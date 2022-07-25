/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {

    // from the wikipedia page
    if num_discs == 0 {
        return vec![];
    }

    //  Move n - 1 disks from source to auxiliary, so they are out of the way
    let mut on_aux = hanoi(num_discs - 1, src, dst, aux);
    on_aux.push((src, dst));

    // Move the n - 1 disks that we left on auxiliary onto target
    let mut on_dst = hanoi(num_discs - 1, aux, src, dst);
    on_aux.append(&mut on_dst);
    on_aux
}