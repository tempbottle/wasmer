use super::process::abort_with_message;
use wasmer_runtime_core::vm::Ctx;

pub fn nullfunc_i(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_i {}", x);
    abort_with_message("Invalid function pointer called with signature 'i'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_ii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_ii {}", x);
    abort_with_message("Invalid function pointer called with signature 'ii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_iii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_iii {}", x);
    abort_with_message("Invalid function pointer called with signature 'iii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_iiii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_iiii {}", x);
    abort_with_message("Invalid function pointer called with signature 'iiii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_iiiii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_iiiii {}", x);
    abort_with_message("Invalid function pointer called with signature 'iiiii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_iiiiii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_iiiiii {}", x);
    abort_with_message("Invalid function pointer called with signature 'iiiiii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_v(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_v {}", x);
    abort_with_message("Invalid function pointer called with signature 'v'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_vi(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_vi {}", x);
    abort_with_message("Invalid function pointer called with signature 'vi'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_vii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_vii {}", x);
    abort_with_message("Invalid function pointer called with signature 'vii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_viii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_viii {}", x);
    abort_with_message("Invalid function pointer called with signature 'viii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_viiii(x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_viiii {}", x);
    abort_with_message("Invalid function pointer called with signature 'viiii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_viiiii(_x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_viiiii");
    abort_with_message("Invalid function pointer called with signature 'viiiii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}

pub fn nullfunc_viiiiii(_x: u32, ctx: &mut Ctx) {
    debug!("emscripten::nullfunc_viiiiii");
    abort_with_message("Invalid function pointer called with signature 'viiiiii'. Perhaps this is an invalid value (e.g. caused by calling a virtual method on a NULL pointer)? Or calling a function with an incorrect type, which will fail? (it is worth building your source files with -Werror (warnings are errors), as warnings can indicate undefined behavior which can cause this)", ctx);
}
