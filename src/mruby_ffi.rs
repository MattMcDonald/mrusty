// mrusty. mruby bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::ffi::CStr;
use std::mem;

pub enum MRState {}

pub enum MRContext {}
pub enum MRParser {}

pub enum MRProc {}
pub enum MRClass {}
pub enum MRObject {}

type MRFunc = extern "C" fn(*mut MRState, MRValue) -> MRValue;

#[repr(C)]
struct RustType {
    pub ptr: *const u8,
    pub size: usize
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MRValue {
    pub value: [u8; 8],
    pub typ: MRType
}

impl MRValue {
    pub fn empty() -> MRValue {
        MRValue {
            value: [0; 8],
            typ: MRType::MRB_TT_FALSE
        }
    }

    pub unsafe fn nil() -> MRValue {
        mrb_ext_nil()
    }

    pub unsafe fn bool(value: bool) -> MRValue {
        if value {
            mrb_ext_true()
        } else {
            mrb_ext_false()
        }
    }

    pub unsafe fn fixnum(value: i32) -> MRValue {
        mrb_ext_cint_to_fixnum(value)
    }

    pub unsafe fn float(mrb: *mut MRState, value: f64) -> MRValue {
        mrb_ext_cdouble_to_float(mrb, value)
    }

    pub unsafe fn str(mrb: *mut MRState, value: &str) -> MRValue {
        mrb_str_new_cstr(mrb, value.as_ptr())
    }

    pub unsafe fn prc(mrb: *mut MRState, value: *mut MRProc) -> MRValue {
        mrb_ext_proc_to_value(mrb, value)
    }

    pub unsafe fn obj<T>(mrb: *mut MRState, obj: &T) -> MRValue {
        let ptr: *const T = obj;
        let ptr: *const u8 = ptr as *const u8;

        mrb_ext_rust_to_ptr(mrb, ptr, mem::size_of::<T>())
    }

    pub unsafe fn to_bool(&self) -> Result<bool, &str> {
        match self.typ {
            MRType::MRB_TT_FALSE => Ok(false),
            MRType::MRB_TT_TRUE  => Ok(true),
            _ => Err("Value must be Fixnum.")
        }
    }

    pub unsafe fn to_i32(&self) -> Result<i32, &str> {
        match self.typ {
            MRType::MRB_TT_FIXNUM => {
                Ok(mrb_ext_fixnum_to_cint(*self))
            },
            _ => Err("Value must be Fixnum.")
        }
    }

    pub unsafe fn to_f64(&self) -> Result<f64, &str> {
        match self.typ {
            MRType::MRB_TT_FLOAT => {
                Ok(mrb_ext_float_to_cdouble(*self))
            },
            _ => Err("Value must be Float.")
        }
    }

    pub unsafe fn to_str(&self, mrb: *mut MRState) -> Result<&str, &str> {
        match self.typ {
            MRType::MRB_TT_STRING => {
                let s = mrb_str_to_cstr(mrb, *self) as *const i8;

                Ok(CStr::from_ptr(s).to_str().unwrap().clone())
            },
            _ => Err("Value must be String.")
        }
    }

    pub unsafe fn to_prc(&self) -> Result<*mut MRProc, &str> {
        match self.typ {
            MRType::MRB_TT_PROC => {
                Ok(mrb_ext_value_to_proc(*self))
            },
            _ => Err("Value must be Proc.")
        }
    }

    pub unsafe fn to_obj<T: Copy>(&self) -> Result<T, &str> {
        match self.typ {
            MRType::MRB_TT_CPTR => {
                let obj = mrb_ext_ptr_to_rust(*self);

                let ptr: *const T = obj.ptr as *const T;
                let ptr = mem::transmute::<*const T, &T>(ptr);

                let obj = *ptr;

                mrb_ext_free_rust(*self);

                Ok(obj)
            },
            _ => Err("Value must be C pointer.")
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum MRType {
    MRB_TT_FALSE,
    MRB_TT_FREE,
    MRB_TT_TRUE,
    MRB_TT_FIXNUM,
    MRB_TT_SYMBOL,
    MRB_TT_UNDEF,
    MRB_TT_FLOAT,
    MRB_TT_CPTR,
    MRB_TT_OBJECT,
    MRB_TT_CLASS,
    MRB_TT_MODULE,
    MRB_TT_ICLASS,
    MRB_TT_SCLASS,
    MRB_TT_PROC,
    MRB_TT_ARRAY,
    MRB_TT_HASH,
    MRB_TT_STRING,
    MRB_TT_RANGE,
    MRB_TT_EXCEPTION,
    MRB_TT_FILE,
    MRB_TT_ENV,
    MRB_TT_DATA,
    MRB_TT_FIBER,
    MRB_TT_MAXDEFINE
}

#[link(name = "mruby")]
#[link(name = "mrbe")]
extern "C" {
    pub fn mrb_open() -> *mut MRState;
    pub fn mrb_close(mrb: *mut MRState);

    pub fn mrbc_context_new(mrb: *mut MRState) -> *mut MRContext;

    pub fn mrbc_filename(mrb: *mut MRState, context: *mut MRContext, filename: *const u8) -> *const u8;

    pub fn mrb_parse_string(mrb: *mut MRState, code: *const u8, context: *mut MRContext) -> *mut MRParser;
    pub fn mrb_generate_code(mrb: *mut MRState, parser: *mut MRParser) -> *mut MRProc;

    pub fn mrb_load_string_cxt(mrb: *mut MRState, code: *const u8, context: *mut MRContext) -> MRValue;

    pub fn mrb_top_self(mrb: *mut MRState) -> MRValue;
    pub fn mrb_run(mrb: *mut MRState, prc: *mut MRProc, value: MRValue) -> MRValue;

    pub fn mrb_class_defined(mrb: *mut MRState, name: *const u8) -> u8;
    pub fn mrb_class_get(mrb: *mut MRState, name: *const u8) -> *mut MRClass;
    pub fn mrb_module_get(mrb: *mut MRState, name: *const u8) -> *mut MRClass;

    pub fn mrb_define_class(mrb: *mut MRState, name: *const u8, sup: *mut MRClass) -> *mut MRClass;
    pub fn mrb_define_module(mrb: *mut MRState, name: *const u8) -> *mut MRClass;

    pub fn mrb_include_module(mrb: *mut MRState, module: *mut MRClass, incl: *mut MRClass);
    pub fn mrb_prepend_module(mrb: *mut MRState, module: *mut MRClass, prep: *mut MRClass);

    pub fn mrb_define_method(mrb: *mut MRState, class: *mut MRClass, name: *const u8, fun: MRFunc, aspec: u32);
    pub fn mrb_define_class_method(mrb: *mut MRState, class: *mut MRClass, name: *const u8, fun: MRFunc, aspec: u32);
    pub fn mrb_define_module_function(mrb: *mut MRState, module: *mut MRClass, name: *const u8, fun: MRFunc, aspec: u32);

    pub fn mrb_obj_new(mrb: *mut MRState, class: *mut MRClass, argc: i32, argv: *const MRValue) -> MRValue;

    pub fn mrb_proc_new_cfunc(mrb: *mut MRState, fun: MRFunc) -> *mut MRProc;

    pub fn mrb_get_args(mrb: *mut MRState, format: *const u8, ...);
    pub fn mrb_yield_argv(mrb: *mut MRState, prc: MRValue, argc: i32, argv: *const MRValue) -> MRValue;

    pub fn mrb_intern_cstr(mrb: *mut MRState, string: *const u8) -> u32;

    pub fn mrb_funcall(mrb: *mut MRState, object: MRValue, name: *const u8, argc: i32, ...) -> MRValue;
    pub fn mrb_funcall_with_block(mrb: *mut MRState, object: MRValue, sym: u32, argc: i32, argv: *const MRValue, prc: MRValue) -> MRValue;

    pub fn mrb_ext_fixnum_to_cint(value: MRValue) -> i32;
    pub fn mrb_ext_float_to_cdouble(value: MRValue) -> f64;
    pub fn mrb_ext_value_to_proc(value: MRValue) -> *mut MRProc;
    pub fn mrb_ext_ptr_to_rust(value: MRValue) -> RustType;

    pub fn mrb_ext_free_rust(value: MRValue);

    pub fn mrb_ext_nil() -> MRValue;
    pub fn mrb_ext_false() -> MRValue;
    pub fn mrb_ext_true() -> MRValue;
    pub fn mrb_ext_cint_to_fixnum(value: i32) -> MRValue;
    pub fn mrb_ext_cdouble_to_float(mrb: *mut MRState, value: f64) -> MRValue;
    pub fn mrb_str_new_cstr(mrb: *mut MRState, value: *const u8) -> MRValue;
    pub fn mrb_ext_proc_to_value(mrb: *mut MRState, prc: *mut MRProc) -> MRValue;
    pub fn mrb_ext_rust_to_ptr(mrb: *mut MRState, ptr: *const u8, size: usize) -> MRValue;

    pub fn mrb_str_to_cstr(mrb: *mut MRState, value: MRValue) -> *const u8;

    pub fn mrb_ext_get_exc(mrb: *mut MRState) -> MRValue;
}


#[path="tests/mruby_ffi.rs"]
#[cfg(test)]
mod tests;
