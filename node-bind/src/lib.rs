// #![deny(clippy::all)]

// #[macro_use]
// extern crate napi_derive;

// use napi::{CallContext, Env, JsNumber, JsObject, JsString, Property, Result, Task};
// use std::convert::TryInto;

// use engine::core::{Engine, PersistMode};

// struct AsyncTask(u32);

// impl Task for AsyncTask {
//     type Output = u32;
//     type JsValue = JsNumber;

//     fn compute(&mut self) -> Result<Self::Output> {
//         use std::thread::sleep;
//         use std::time::Duration;
//         sleep(Duration::from_millis(self.0 as u64));
//         Ok(self.0 * 2)
//     }

//     fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
//         env.create_uint32(output)
//     }
// }

// #[js_function(1)]
// fn new_engine(ctx: CallContext) -> Result<JsObject> {
//     let engine_params: JsObject = ctx.get::<JsObject>(0)?;
//     let persist_mode: PersistMode = engine_params
//         .get_named_property::<JsString>("persist_mode")?
//         .into_utf8()?
//         .as_str()?
//         .parse()
//         .unwrap();

//     let engine = Engine::new(persist_mode, None);
//     let mut js_engine: JsObject = ctx.this_unchecked();
//     ctx.env.wrap(&mut js_engine, engine)?;
//     Ok(js_engine)
// }

// #[js_function(1)]
// fn create_process(ctx: CallContext) -> Result<JsString> {
//     // let mut engineParams: JsObject = ctx.get::<JsObject>(0)?;
//     // println!("{:?}", engine);
//     println!("start createProcess");
//     let this: JsObject = ctx.this_unchecked();
//     let engine: &mut Engine = ctx.env.unwrap(&this)?;
//     let uuid = engine.create_process();
//     // ctx.env.wrap(&mut jsEngine, engine);
//     ctx.env.create_string(uuid.to_string().as_str())
// }

// // #[js_function(1)]
// // fn start_process(ctx: CallContext) -> Result<JsObject> {
// //     // let engineParams: JsObject = ctx.get::<JsObject>(0)?;
// //     // println!("{:?}", engine);
// //     println!("start process");
// //     let js_engine: JsObject = ctx.this_unchecked();
// //     // ctx.env.wrap(&mut jsEngine, engine);
// //     Ok(js_engine)
// // }

// #[module_exports]
// fn init(mut exports: JsObject, env: Env) -> Result<()> {
//     let engine_class = env.define_class(
//         "Engine",
//         new_engine,
//         &[
//             Property::new(&env, "createProcess")?.with_method(create_process),
//             // Property::new(&env, "startProcess")?.with_method(start_process),
//         ],
//     )?;
//     exports.set_named_property("Engine", engine_class)?;
//     Ok(())
// }

// #[js_function(1)]
// fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
//     let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;

//     ctx.env.create_uint32(argument + 100)
// }

// #[js_function(1)]
// fn sleep(ctx: CallContext) -> Result<JsObject> {
//     let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
//     let task = AsyncTask(argument);
//     let async_task = ctx.env.spawn(task)?;
//     Ok(async_task.promise_object())
// }
