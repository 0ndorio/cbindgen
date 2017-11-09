type A = fn ();
type B = fn () -> ();
type C = fn (i32, i32) -> bool;
type D = fn (i32) -> fn (f32) -> bool;
type E = fn () -> *const [i32; 16];

type F = *const i32;
type G = *const *const i32;
type H = *const *mut i32;
type I = *const [i32; 16];
type J = *const fn (f32) -> f64;

type K = [i32; 16];
type L = [*const i32; 16];
type M = [fn (i32, i32) -> bool; 16];
type N = [fn (i32, i32) -> (); 16];

#[no_mangle]
extern "C" fn O() -> fn ()
{ }

#[no_mangle]
extern "C" fn root(a: A,
                   b: B,
                   c: C,
                   d: D,
                   e: E,
                   f: F,
                   g: G,
                   h: H,
                   i: I,
                   j: J,
                   k: K,
                   l: L,
                   m: M,
                   n: N)
{ }
