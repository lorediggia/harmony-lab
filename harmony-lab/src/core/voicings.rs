use std::collections::HashSet;

pub const OPEN_PCS: [usize; 6] = [4, 9, 2, 7, 11, 4]; // E A D G B E 

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Voicing {
    pub frets: [Option<u8>; 6],
}

impl Voicing {
    pub fn lowest_fret(&self) -> u8 {
        self.frets.iter().flatten().filter(|&&f| f > 0).copied().min().unwrap_or(0)
    }
    pub fn span(&self) -> u8 {
        let f: Vec<u8> = self.frets.iter().flatten().filter(|&&f| f > 0).copied().collect();
        if f.is_empty() { 0 } else { f.iter().max().unwrap() - f.iter().min().unwrap() }
    }
    pub fn strings_played(&self) -> usize {
        self.frets.iter().filter(|f| f.is_some()).count()
    }
    pub fn bass_pc(&self) -> Option<usize> {
        for (i, f) in self.frets.iter().enumerate() {
            if let Some(fret) = f { return Some((OPEN_PCS[i] + *fret as usize) % 12); }
        }
        None
    }
}

#[derive(Clone, Copy)]
struct Shape {
    pattern: [Option<u8>; 6],
    root_string: usize,
    root_offset: u8,
}

impl Shape {
    fn apply(&self, target_root_pc: usize) -> Option<Voicing> {
        let tmpl_root_pc = (OPEN_PCS[self.root_string] + self.root_offset as usize) % 12;
        let shift = (target_root_pc + 12 - tmpl_root_pc) % 12;
        let mut frets = [None; 6];
        for i in 0..6 {
            if let Some(p) = self.pattern[i] {
                let new_fret = p as usize + shift;
                if new_fret > 22 { return None; }
                frets[i] = Some(new_fret as u8);
            }
        }
        Some(Voicing { frets })
    }
}

// Maggiore (1-3-5)
const MAJOR: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(2), Some(1), Some(0), Some(0)], root_string: 0, root_offset: 0 }, // E (es. F barré 133211)
    Shape { pattern: [None,    Some(0), Some(2), Some(2), Some(2), Some(0)], root_string: 1, root_offset: 0 }, // A (es. B barré x24442)
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(3), Some(2)], root_string: 2, root_offset: 0 }, // D (xx0232)
    Shape { pattern: [None,    Some(3), Some(2), Some(0), Some(1), Some(0)], root_string: 1, root_offset: 3 }, // C (x32010)
    Shape { pattern: [Some(3), Some(2), Some(0), Some(0), Some(0), Some(3)], root_string: 0, root_offset: 3 }, // G (320003)
];

// Minore (1-b3-5)
const MINOR: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(2), Some(0), Some(0), Some(0)], root_string: 0, root_offset: 0 }, // Em (022000)
    Shape { pattern: [None,    Some(0), Some(2), Some(2), Some(1), Some(0)], root_string: 1, root_offset: 0 }, // Am (x02210)
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(3), Some(1)], root_string: 2, root_offset: 0 }, // Dm (xx0231)
];

// 7 dominante (1-3-5-b7)
const DOM7: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(0), Some(1), Some(0), Some(0)], root_string: 0, root_offset: 0 }, // E7 (020100)
    Shape { pattern: [None,    Some(0), Some(2), Some(0), Some(2), Some(0)], root_string: 1, root_offset: 0 }, // A7 (x02020)
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(1), Some(2)], root_string: 2, root_offset: 0 }, // D7 (xx0212)
    Shape { pattern: [None,    Some(3), Some(2), Some(3), Some(1), Some(0)], root_string: 1, root_offset: 3 }, // C7 (x32310)
    Shape { pattern: [Some(3), Some(2), Some(0), Some(0), Some(0), Some(1)], root_string: 0, root_offset: 3 }, // G7 (320001)
];

// Maj7 (1-3-5-7)
const MAJ7: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(1), Some(1), Some(0), Some(0)], root_string: 0, root_offset: 0 }, // Emaj7 (021100)
    Shape { pattern: [None,    Some(0), Some(2), Some(1), Some(2), Some(0)], root_string: 1, root_offset: 0 }, // Amaj7 (x02120)
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(2), Some(2)], root_string: 2, root_offset: 0 }, // Dmaj7 (xx0222)
    Shape { pattern: [None,    Some(3), Some(2), Some(0), Some(0), Some(0)], root_string: 1, root_offset: 3 }, // Cmaj7 (x32000)
    Shape { pattern: [Some(3), Some(2), Some(0), Some(0), Some(0), Some(2)], root_string: 0, root_offset: 3 }, // Gmaj7 (320002)
];

// m7 (1-b3-5-b7)
const MIN7: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(0), Some(0), Some(0), Some(0)], root_string: 0, root_offset: 0 }, // Em7 (020000)
    Shape { pattern: [None,    Some(0), Some(2), Some(0), Some(1), Some(0)], root_string: 1, root_offset: 0 }, // Am7 (x02010)
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(1), Some(1)], root_string: 2, root_offset: 0 }, // Dm7 (xx0211)
];

// m7b5 / semi-diminuito (1-b3-b5-b7)
const M7B5: &[Shape] = &[
    Shape { pattern: [Some(0), Some(1), Some(2), Some(0), Some(3), Some(0)], root_string: 0, root_offset: 0 }, // Em7b5 (012030)
    Shape { pattern: [None,    Some(0), Some(1), Some(0), Some(1), None    ], root_string: 1, root_offset: 0 }, // Am7b5 (x0101x)
    Shape { pattern: [None,    None,    Some(0), Some(1), Some(1), Some(1)], root_string: 2, root_offset: 0 }, // Dm7b5 (xx0111)
];

// dim7 (1-b3-b5-bb7)
const DIM7: &[Shape] = &[
    Shape { pattern: [None,    Some(2), Some(3), Some(1), Some(3), Some(1)], root_string: 1, root_offset: 2 }, // root su A
    Shape { pattern: [Some(0), Some(1), Some(2), Some(0), Some(2), Some(0)], root_string: 0, root_offset: 0 }, // Edim7 type
];

// Aumentato (1-3-#5)
const AUG: &[Shape] = &[
    Shape { pattern: [Some(0), Some(3), Some(2), Some(1), Some(1), Some(0)], root_string: 0, root_offset: 0 }, // Eaug (032110)
];

// sus2 (1-2-5)
const SUS2: &[Shape] = &[
    Shape { pattern: [None,    Some(0), Some(2), Some(2), Some(0), Some(0)], root_string: 1, root_offset: 0 }, // Asus2 (x02200)
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(3), Some(0)], root_string: 2, root_offset: 0 }, // Dsus2 (xx0230)
];

// sus4 (1-4-5)
const SUS4: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(2), Some(2), Some(0), Some(0)], root_string: 0, root_offset: 0 }, // Esus4 (022200)
    Shape { pattern: [None,    Some(0), Some(2), Some(2), Some(3), Some(0)], root_string: 1, root_offset: 0 }, // Asus4 (x02230)
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(3), Some(3)], root_string: 2, root_offset: 0 }, // Dsus4 (xx0233)
];

// 7sus4 (1-4-5-b7)
const SEVEN_SUS4: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(0), Some(2), Some(0), Some(0)], root_string: 0, root_offset: 0 }, // E7sus4
    Shape { pattern: [None,    Some(0), Some(2), Some(0), Some(3), Some(0)], root_string: 1, root_offset: 0 }, // A7sus4
    Shape { pattern: [None,    None,    Some(0), Some(2), Some(1), Some(3)], root_string: 2, root_offset: 0 }, // D7sus4
];

// Power chord (1-5) 
const POWER: &[Shape] = &[
    Shape { pattern: [Some(0), Some(2), Some(2), None, None, None], root_string: 0, root_offset: 0 }, // E5
    Shape { pattern: [None, Some(0), Some(2), Some(2), None, None], root_string: 1, root_offset: 0 }, // A5
    Shape { pattern: [None, None, Some(0), Some(2), Some(3), None], root_string: 2, root_offset: 0 }, // D5
];

fn quality_signature(pcs: &[usize]) -> Vec<usize> {
    if pcs.is_empty() { return vec![]; }
    let root = pcs[0];
    let mut intervals: Vec<usize> = pcs.iter().map(|&p| (p + 12 - root) % 12).collect();
    intervals.sort();
    intervals.dedup();
    intervals
}

fn shapes_for(intervals: &[usize]) -> Option<&'static [Shape]> {
    match intervals {
        [0, 4, 7]           => Some(MAJOR),
        [0, 3, 7]           => Some(MINOR),
        [0, 4, 8]           => Some(AUG),
        [0, 2, 7]           => Some(SUS2),
        [0, 5, 7]           => Some(SUS4),
        [0, 7]              => Some(POWER),
        [0, 4, 7, 10]       => Some(DOM7),
        [0, 4, 7, 11]       => Some(MAJ7),
        [0, 3, 7, 10]       => Some(MIN7),
        [0, 3, 6, 10]       => Some(M7B5),
        [0, 3, 6, 9]        => Some(DIM7),
        [0, 5, 7, 10]       => Some(SEVEN_SUS4),
        _ => None,
    }
}

pub fn generate_voicings(chord_pcs: &[usize], root_pc: usize) -> Vec<Voicing> {
    let intervals = quality_signature(chord_pcs);

    if let Some(shapes) = shapes_for(&intervals) {
        let mut result: Vec<Voicing> = shapes.iter()
            .filter_map(|s| s.apply(root_pc))
            .filter(|v| is_playable(v))
            .collect();
        result.sort_by_key(|v| v.lowest_fret());
        result.dedup();
        return result;
    }

    algorithmic_voicings(chord_pcs, root_pc)
}

fn is_playable(v: &Voicing) -> bool {
    let f: Vec<u8> = v.frets.iter().flatten().filter(|&&f| f > 0).copied().collect();
    if f.is_empty() { return true; }
    let min = *f.iter().min().unwrap();
    let max = *f.iter().max().unwrap();
    if max - min > 4 { return false; }
    let above = f.iter().filter(|&&x| x > min).count();
    above + 1 <= 5
}

fn algorithmic_voicings(chord_pcs: &[usize], root_pc: usize) -> Vec<Voicing> {
    let pcs: HashSet<usize> = chord_pcs.iter().copied().collect();
    let mut all = Vec::new();

    for start_fret in 0u8..=18 {
        let opts = build_opts(&pcs, start_fret);
        let mut current = [None; 6];
        cartesian(&opts, 0, &mut current, &mut |combo| {
            let v = Voicing { frets: *combo };
            if is_valid(&v, &pcs, root_pc) { all.push(v); }
        });
    }

    all.sort_by_key(|v| v.lowest_fret());
    all.dedup();

    let mut used: HashSet<u8> = HashSet::new();
    let mut kept = Vec::new();
    for v in all {
        if used.insert(v.lowest_fret() / 2) {
            kept.push(v);
            if kept.len() >= 6 { break; }
        }
    }
    kept
}

fn build_opts(pcs: &HashSet<usize>, start: u8) -> [Vec<Option<u8>>; 6] {
    let mut opts: [Vec<Option<u8>>; 6] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for s in 0..6 {
        let open_pc = OPEN_PCS[s];
        opts[s].push(None);
        if pcs.contains(&open_pc) { opts[s].push(Some(0)); }
        for f in start.max(1)..=(start + 4).min(22) {
            let pc = (open_pc + f as usize) % 12;
            if pcs.contains(&pc) { opts[s].push(Some(f)); }
        }
    }
    opts
}

fn cartesian<F: FnMut(&[Option<u8>; 6])>(opts: &[Vec<Option<u8>>; 6], idx: usize, cur: &mut [Option<u8>; 6], cb: &mut F) {
    if idx == 6 { cb(cur); return; }
    for o in &opts[idx] { cur[idx] = *o; cartesian(opts, idx + 1, cur, cb); }
}

fn is_valid(v: &Voicing, pcs: &HashSet<usize>, root_pc: usize) -> bool {
    if v.strings_played() < 3 { return false; }
    let played: HashSet<usize> = v.frets.iter().enumerate()
        .filter_map(|(i, f)| f.map(|fr| (OPEN_PCS[i] + fr as usize) % 12)).collect();
    if !pcs.is_subset(&played) || !played.is_subset(pcs) { return false; }
    if v.bass_pc() != Some(root_pc) { return false; }
    if has_gap(v) { return false; }
    is_playable(v)
}

fn has_gap(v: &Voicing) -> bool {
    let mut started = false;
    let mut had_mute = false;
    for f in &v.frets {
        if f.is_some() { if had_mute { return true; } started = true; }
        else if started { had_mute = true; }
    }
    false
}