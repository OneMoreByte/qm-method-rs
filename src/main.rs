use std::io;


/// Term struct to store the m terms, the current term in a sudo binary formate and
/// a bool to check if it has been matched this loop
#[derive(Clone, Debug)]
struct Term {
    binary_eqv: Vec<i8>,
    m_terms: Vec<u64>,
    has_matched: bool,
}

// Functions for struct
impl Term {
    /// I thought this was kind of like an xor gate. It counts the differences between the two
    /// and returns it
    fn xor_and_sum(&self, b: &Term) -> u64 {
        let mut sum: u64 = 0;
        let mut itr = self.binary_eqv.iter().zip(b.binary_eqv.iter()); // This combines two iterators as one that returns a pair (A,B) with each next
        let mut current_term = itr.next();
        while current_term.is_some() {
            let (x, y) = current_term.unwrap(); // I think the proper name is tuplet?
            if x != y {
                sum += 1;
            }

            current_term = itr.next();

        }
        (sum)
    }

    /*** Don't feed this anything that hasn't been confirmed as ready to combine ***/
    /// Combines two terms into one
    fn combine(a: &Term, b: &Term) -> Term {
        let mut z = Term {
            binary_eqv: Vec::new(),
            m_terms: Vec::new(),
            has_matched: false,
        };

        let mut itr = a.binary_eqv.iter().zip(b.binary_eqv.iter()); // This combines two iterators as one that returns a pair (A,B) with each next
        let mut current_term = itr.next();

        while current_term.is_some() {

            let (x, y) = current_term.unwrap();
            if x != y {
                z.binary_eqv.push(2);
            } else {
                z.binary_eqv.push(x.clone());
            }

            current_term = itr.next();
        }

        let mut m_together = a.m_terms.iter().chain(b.m_terms.iter()); // This appends one iterator to another
        let mut current_term = m_together.next();

        while current_term.is_some() {
            z.m_terms.push(current_term.unwrap().clone());
            current_term = m_together.next();
        }


        (z)
    }

    /// Inintializes struct. Takes the m term and the number of terms as input.
    fn new(m: u64, n_terms: i8) -> Term {
        let mut z = Term {
            binary_eqv: Vec::new(),
            m_terms: Vec::new(),
            has_matched: false,
        };
        let t: u64 = 2;
        for x in 0..n_terms {
            z.binary_eqv.push((((m / (t.pow(x as u32))) % 2) as i8))
        }
        z.binary_eqv.reverse();
        z.m_terms.push(m);
        (z)
    }

    /// returns the number of ones in binary_eqv, Used for sorting when factoring
    fn count_ones(&self) -> i8 {
        let mut ones = 0;

        for n in &self.binary_eqv {
            if 1 == *n {
                ones += 1;
            }
        }
        (ones)
    }

    /// I was having some problems changing the bool. I was just stupid. This really shouldn't exist
    fn change_match(&mut self) {
        self.has_matched = true;
    }

    /// Given the current position, subtracts one for each already used m term, and adds for each unused
    /// Returns that sum, which is used at the end to find all the nessisary terms
    fn calc_value(&self, pos: usize) -> i8 {
        let mut value = 0;
        for a in &self.m_terms {
            if pos < (*a as usize) {
                value += 1;
            } else {
                value += -1;
            }
        }
        (value)
    }

    /// returns all the m terms that the Term has that haven't been encountered starting at position p
    fn get_otherm(&self, p: usize) -> Vec<u64> {
        let mut out: Vec<u64> = Vec::new();
        for m in &self.m_terms {
            if (*m as usize) > p {
                out.push(m.clone());
            }
        }
        (out)
    }

    /// There must be a better waaaay
    /// Spits out a string version of binary_eqv that is pretty and human readable
    fn pretty_term(&self) -> String {
        let c = [
            'A',
            'B',
            'C',
            'D',
            'E',
            'F',
            'G',
            'H',
            'I',
            'J',
            'K',
            'L',
            'M',
            'N',
            'O',
            'P',
            'Q',
            'R',
            'T',
            'U',
            'V',
            'Q',
            'X',
            'Y',
            'Z',
            'a',
            'b',
            'c',
            'd',
            'e',
            'f',
            'g',
            'h',
            'i',
            'j',
            'k',
            'l',
            'm',
            'n',
            'o',
            'p',
            'q',
            'r',
            's',
            't',
            'u',
            'v',
            'q',
            'x',
            'y',
            'z',
        ];

        let mut s = "".to_string();
        let mut i: usize = 0;
        for b in &self.binary_eqv {
            match *b {
                0 => s.push_str(&format!("{0}'", c[i])),
                1 => s.push(c[i]),
                2 => i = i,
                _ => s.push('#'),

            }
            i += 1;
        }

        (s)
    }
}

/// gets a string from the terminal
fn get_input_string() -> Option<String> {
    let mut buf = String::new();
    let stdin = io::stdin();

    match stdin.read_line(&mut buf) {
        Ok(_n) => Some(buf),
        Err(_error) => None,
    }
}

/// Gets the number of terms
/// Error handeling!
fn get_terms() -> i8 {
    let mut terms;
    println!("How many terms?");
    let mut raw = get_input_string();

    while raw.is_none() {
        println!("That's somehow invalid. Try again?");
        raw = get_input_string();
    }

    let input = raw.unwrap();
    let output = input.trim().parse::<i8>();

    // match is rust's switch This is checking for errors parsing the int
    match output {
        Err(e) => {
            println!("That didn't work, try again. {:?}", e);
            terms = get_terms();
        }
        Ok(a) => terms = a,
    }

    // Check input, if it's bad, run again for good input
    if terms > 52 {
        println!("The max is 52 terms. Please enter a number lower than that.");
        terms = get_terms();
    } else if terms < 2 {
        print!("The minimum is 2 terms. You", );
        if terms == 1 {
            println!(" don't really need to simplify a 1 term expression :P");
            terms = get_terms();
        } else {
            println!(
                " can't really have an expression with {:?} terms now can you. :)",
                terms
            );
            terms = get_terms();
        }
    }

    (terms)
}

/// given the borrowed 2d vector (bin), and an m term, and number of terms, create and add the Term to bin
fn place_mterm(mut bin: &mut Vec<Vec<Term>>, m: u64, n: i8) {
    let term = Term::new(m, n);
    let i = term.count_ones();
    let mut vc = bin.remove(i as usize);
    vc.push(term);
    bin.insert(i as usize, vc);
}

/// Inintializes Vec<Vec<Term>> with the user's input
fn get_bin(n_terms: i8) -> Vec<Vec<Term>> {
    let mut bin: Vec<Vec<Term>> = Vec::new();

    for _x in 0..(n_terms + 1) {
        bin.push(Vec::new());
    }

    println!("Please input the minterms like so \"1, 6, 7\"");
    let mut raw = get_input_string();

    while raw.is_none() {
        println!("That's somehow invalid. Try again?");
        raw = get_input_string();
    }
    let temp = raw.unwrap();
    // Break input into peices, trim them, and put their Term equivlient into bin
    let terms: Vec<&str> = temp.split(',').collect();

    for t in terms {
        match t.trim().parse::<u64>() {
            Ok(a) => {
                place_mterm(&mut bin, a, n_terms);
            }
            Err(e) => {
                println!("The term {:?} is invalid. : {:?}", t, e);
            }
        }
    }

    (bin)
}

/// Gets all the factors we cannn
/// I think this could be reimplemented to be quicker and simpler
fn find_factors(bin: &Vec<Vec<Term>>, n: i8) -> Vec<Vec<Term>> {

    /*
    println!("Printing input", );
    for a in bin {
        for b in a {
            println!("{0}", b.pretty_term());
        }
    }
    println!("Done", );
    */


    let mut slot: Vec<Term>;
    let mut slot_a: Vec<Term>;
    let mut boox: Vec<Vec<Term>> = Vec::new(); // This is here because I don't fully understand rust's borrowing. It IS bin
    let mut temp: Vec<Vec<Term>> = Vec::new();
    let mut still_matching = false;

    for c in bin {
        temp.push(Vec::new());
        boox.push(c.to_vec());
    }

    //This pulls two Vec's from the Vec<Vec>, then goes through the last most vec to see if there are any to factor
    for x in 1..(n + 1) {
        slot_a = boox.remove((x - 1) as usize);
        slot = boox.remove((x - 1) as usize);

        for t in &mut slot_a {
            for o in &mut slot {
                if t.xor_and_sum(&o) == 1 {
                    // If there are we combine them, flip the boolean, and push into a temp Vec<Vec> we return
                    let term = Term::combine(t, o);
                    temp[term.count_ones() as usize].push(term);
                    still_matching = true;
                    t.change_match();
                    o.change_match();
                }
            }

        }
        // Any that didn't get matched also have to be added
        for t in &mut slot_a {
            if !t.has_matched {
                temp[t.count_ones() as usize].push(t.clone());
            }
        }

        // And we put everything back
        boox.insert((x - 1) as usize, slot_a);
        boox.insert(x as usize, slot);

    }

    // We switch everything back to false for next ittereation
    for a in &mut temp {
        for b in a {
            b.has_matched = false;
        }
    }

    // If we never found a match, return, else run again with new input
    if still_matching {
        (find_factors(&temp, n))
    } else {

        (temp)
    }
}

/// Finds solution and prints it
fn find_solution(bin: Vec<Vec<Term>>, terms: i8) {

    let mut graph: Vec<Vec<&Term>> = Vec::new();
    let mut done: Vec<&Term> = Vec::new();
    let mut accounted_for: Vec<u64> = Vec::new();

    // We're making a crazy (kind of inefficent I feel like) Vec of references.
    for _a in 0..((2 as i8).pow(terms as u32)) {
        graph.push(Vec::new());
    }

    // And we put terms in every position that they have a m term for
    for rows in &bin {
        for col in rows {
            if col.m_terms.len() > 1 {
                for m in 0..(col.m_terms.len()) {
                    graph[col.m_terms[m] as usize].push(&col);
                }
            } else {
                graph[col.m_terms[0] as usize].push(&col);
            }
        }
    }

    // Then we go though it, and find the ones we actualy want
    let mut pos: usize = 0;
    for g in graph {

        let mut matches = false;
        for a in &accounted_for {
            if pos == *a as usize {
                matches = true;
            }
        }
        if g.len() > 0 && !matches {
            let mut strongest: &Term = g[0];
            for x in g {
                if x.calc_value(pos) > strongest.calc_value(pos) {
                    strongest = x;
                }
            }
            accounted_for.append(&mut strongest.get_otherm(pos));
            accounted_for.sort();
            done.push(strongest);
        }
        pos += 1;
    }
    println!("\n And the output is... \n");
    let mut e = '=';
    print!("F ");
    for t in done {
        print!("{0} ", e);
        print!("{0} ", t.pretty_term());
        print!("{:?} ", (*t).m_terms);

        e = '+';
    }
    print!("\n");

}


/// Main function :)
fn main() {

    let numb_of_terms = get_terms();
    let mut term_bins: Vec<Vec<Term>> = get_bin(numb_of_terms);

    term_bins = find_factors(&term_bins, numb_of_terms);

    find_solution(term_bins, numb_of_terms);
}
