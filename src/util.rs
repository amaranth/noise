// Copyright 2014 Travis Watkins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::cast;

#[inline]
pub fn lerp<F: Float>(f0: F, f1: F, u: F) -> F {
    u * (f1 - f0) + f0
}

#[inline]
pub fn bilerp<F: Float>(f00: F, f10: F, f01: F, f11: F, u: F, v: F) -> F {
    lerp(lerp(f00, f10, u),
         lerp(f01, f11, u), v)
}

#[inline]
pub fn trilerp<F: Float>(f000: F, f100: F, f010: F, f110: F, f001: F, f101: F, f011: F, f111: F, u: F, v: F, w: F) -> F {
    lerp(bilerp(f000, f100, f010, f110, u, v),
         bilerp(f001, f101, f011, f111, u, v),
         w)
}

#[inline]
pub fn scurve3<F: Float>(t: F) -> F {
    let three : F = cast(3.0_f32).unwrap();
    let two = cast(2.0_f32).unwrap();

    t * t * (three - (t * two))
}

#[inline]
pub fn scurve5<F: Float>(t: F) -> F {
    let six = cast(6.0_f32).unwrap();
    let fifteen = cast(15.0_f32).unwrap();
    let ten = cast(10.0_f32).unwrap();

    t * t * t * (t * (t * six - fifteen) + ten)
}
