use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures::stream::StreamExt;
use futures_channel::mpsc;
use js_sys::{Promise, Uint8ClampedArray, WebAssembly};
use rand::{seq::SliceRandom, thread_rng};
use rayon::{prelude::ParallelIterator, slice::ParallelSlice};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

pub use wasm_bindgen_rayon::init_thread_pool;

mod raytracer;

const CHUNK_SIZE: usize = 500;

type PixelColor = (u32, u32, u8, u8, u8, u8);

#[wasm_bindgen]
extern "C" {
    pub type ImageData;

    #[wasm_bindgen(constructor, catch)]
    fn new(data: &Uint8ClampedArray, width: f64, height: f64) -> Result<ImageData, JsValue>;
}

#[wasm_bindgen]
pub struct Scene {
    scene: raytracer::Scene,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(object: JsValue) -> Result<Scene, JsValue> {
        Ok(Scene {
            scene: serde_wasm_bindgen::from_value(object)
                .map_err(|e| JsValue::from(e.to_string()))?,
        })
    }

    pub fn render(&self, concurrency: usize) -> RenderContext {
        let mut scene = self.scene.clone();
        scene.camera.initialize();

        let width = self.scene.width;
        let height = self.scene.height;

        // NOTE - Generate all pixels and randomly shuffle them.

        let mut rng = thread_rng();

        let x: Vec<u32> = (0..height).collect();
        let y: Vec<u32> = (0..width).collect();

        let mut pixels: Vec<(u32, u32)> = x
            .iter()
            .map(|&item_x| y.iter().map(move |&item_y| (item_x, item_y)))
            .flatten()
            .collect();

        pixels.shuffle(&mut rng);

        // NOTE - Kick off up multi-threaded render.

        let mut data: Vec<u8> = vec![0; 4 * (width as usize) * (height as usize)];

        let base = data.as_ptr() as usize;
        let length = data.len();

        let (tx, mut rx) = mpsc::unbounded::<Vec<PixelColor>>();

        rayon::spawn(move || {
            pixels
                .par_chunks(pixels.len() / concurrency)
                .for_each(|chunk| {
                    let mut tx_clone = tx.clone();

                    chunk.chunks(CHUNK_SIZE).for_each(|inner_chunk| {
                        let pixel_colors: Vec<PixelColor> = inner_chunk
                            .into_iter()
                            .map(|(x, y)| {
                                let (r, g, b) = raytracer::trace_ray(&scene, *x, *y);

                                (*x, *y, r, g, b, 255)
                            })
                            .collect();

                        tx_clone.unbounded_send(pixel_colors).unwrap();
                    });

                    tx_clone.disconnect();
                });
        });

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let done = async move {
            while let Some(pixel_colors) = rx.next().await {
                counter_clone.fetch_add(pixel_colors.len(), Ordering::Relaxed);

                pixel_colors.into_iter().for_each(|(x, y, r, g, b, a)| {
                    let base_index = 4 * (x * width + y) as usize;

                    data[base_index + 0] = r;
                    data[base_index + 1] = g;
                    data[base_index + 2] = b;
                    data[base_index + 3] = a;
                });
            }

            Ok(make_image_data(base, length, width, height).into())
        };

        RenderContext {
            promise: wasm_bindgen_futures::future_to_promise(done),
            base,
            length,
            width,
            height,
            counter,
        }
    }
}

#[wasm_bindgen]
pub struct RenderContext {
    promise: Promise,
    base: usize,
    length: usize,
    width: u32,
    height: u32,
    counter: Arc<AtomicUsize>,
}

#[wasm_bindgen]
impl RenderContext {
    #[wasm_bindgen(js_name = getPromise)]
    pub fn get_promise(&self) -> Promise {
        self.promise.clone()
    }

    #[wasm_bindgen(js_name = getCurrentImageData)]
    pub fn get_current_image_data(&self) -> ImageData {
        make_image_data(self.base, self.length, self.width, self.height)
    }

    #[wasm_bindgen(js_name = getCurrentProgress)]
    pub fn get_current_progress(&self) -> f32 {
        (self.counter.load(Ordering::Relaxed) as f32) / ((self.width * self.height) as f32)
    }
}

fn make_image_data(base: usize, length: usize, width: u32, height: u32) -> ImageData {
    let memory = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
    let data = Uint8ClampedArray::new(&memory.buffer()).slice(base as u32, (base + length) as u32);

    ImageData::new(&data, width as f64, height as f64).unwrap()
}
