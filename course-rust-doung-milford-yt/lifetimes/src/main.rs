/**
 * Rust LifeTimes
 *
 * - Lifetimes were created in Rust to stop the possibility (in compilation time) of you create pointers hanging
 * - All about lifetimes id done by Rust in compiling time
 * - All references in Rust has a lifetime
 * - When the ownership of a memory reach the end of a scope the memory is automaticaly desalocated and cleaned
 * - For every input reference that doesn't have an explicitly defined lifetime, it will be given it's OWN lifetime by Rust in the background
 * - Any reference parameter that is NEVER returned as the output, the Rust provided default lifetime is fine.
 */

fn main() {
    //Transfering the memory allocated
    let a;
    {
        let b = String::from("Lucas");
        // a = &b; //ERROR: Sharing the reference doesn't work because b (ownership of the memory allocated) doesn't live enough to (a) that is out of its scope
        a = b; //Transfering the ownership from b to a
    }

    println!("{}", a);
}
