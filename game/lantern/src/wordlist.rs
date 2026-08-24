//! The course word lists. THIS FILE SHIPS COMPLETE AND IS NEVER EDITED.
//!
//! Two lists, two standards — on purpose:
//!
//! - `WORDS` is the SECRET list: the words the game can pick. It is short and
//!   curated, because a secret has to be worth guessing toward — and because
//!   `words::pick` turns a puzzle number into a position on it, the list is
//!   part of what a puzzle number MEANS. Add a word in the middle and
//!   puzzle 7 becomes a different word on every copy of the game.
//! - `ACCEPTED` is the GUESS dictionary: every secret, plus over a thousand
//!   more common five-letter words. A guess only has to be a real word — a
//!   game that refuses `ghost` because it happens not to be a secret feels
//!   broken to the friend you hand it to.
//!
//! Every word in both lists is exactly five letters, lowercase ASCII `a`–`z`,
//! and appears once; every secret is also an accepted guess. The test at the
//! bottom proves all of that from the lists themselves — no number in this
//! course is typed when it can be measured.
//!
//! Your own code never reads these constants directly. Every function you
//! write takes its words as an argument (`&[&str]`), so your tests can hand it
//! a three-word list they control. Only `main.rs` — the playable, ungraded
//! shell — names either constant.

/// The words the game can pick as its secret.
pub const WORDS: &[&str] = &[
    "crane", "slate", "bread", "house", "plant", "stone", "water", "light",
    "sound", "world", "heart", "dream", "smile", "grape", "lemon", "peach",
    "berry", "apple", "mango", "olive", "wheat", "sugar", "spice", "table",
    "chair", "floor", "brick", "cloud", "storm", "frost", "ember", "flame",
    "river", "ocean", "beach", "shell", "coral", "whale", "tiger", "horse",
    "mouse", "eagle", "robin", "goose", "sheep", "camel", "zebra", "koala",
    "panda", "lunar", "solar", "comet", "orbit", "quiet", "quick", "happy",
    "brave", "sweet", "sharp", "round", "green", "black", "white", "amber",
];

/// The words the game accepts as a guess — every secret above, plus common
/// five-letter words a player will actually try. Kept alphabetical so a human
/// can scan it; the test below is what actually proves its rules.
pub const ACCEPTED: &[&str] = &[
    "about", "above", "actor", "acute", "adapt", "admit", "adopt", "adult",
    "after", "again", "agent", "agree", "ahead", "alarm", "album", "alert",
    "alike", "alive", "allow", "alone", "along", "aloud", "alter", "amber",
    "angel", "anger", "angle", "angry", "ankle", "apart", "apple", "apron",
    "arena", "argue", "arise", "armor", "aroma", "array", "arrow", "aside",
    "asset", "atlas", "audio", "audit", "avoid", "awake", "award", "aware",
    "badge", "badly", "baker", "balmy", "banjo", "basic", "basil", "basin",
    "batch", "beach", "beads", "beast", "began", "begin", "being", "belly",
    "below", "bench", "berry", "birch", "birth", "bison", "black", "blade",
    "blame", "bland", "blank", "blast", "blaze", "bleak", "blend", "bless",
    "blind", "blink", "bliss", "block", "bloom", "blues", "blunt", "blush",
    "board", "boast", "bonus", "boost", "booth", "bound", "brain", "brake",
    "brand", "brass", "brave", "bread", "brick", "bride", "brief", "brine",
    "bring", "brisk", "broad", "broke", "brook", "broom", "brown", "brush",
    "build", "built", "bunch", "bunny", "burnt", "burst", "cabin", "cable",
    "cacao", "cache", "camel", "candy", "canoe", "cargo", "carol", "carry",
    "carve", "catch", "cause", "cedar", "cello", "chain", "chair", "chalk",
    "charm", "chart", "chase", "cheap", "check", "cheek", "cheer", "chess",
    "chest", "chief", "child", "chill", "chime", "china", "chirp", "choir",
    "chord", "chose", "cider", "cinch", "civic", "civil", "claim", "clash",
    "clasp", "class", "clean", "clear", "clerk", "click", "cliff", "climb",
    "cling", "cloak", "clock", "clone", "close", "cloth", "cloud", "clove",
    "clown", "coach", "coast", "cobra", "cocoa", "colon", "color", "comet",
    "comic", "conch", "coral", "corny", "couch", "cough", "could", "count",
    "court", "cover", "craft", "cramp", "crane", "crash", "crate", "crawl",
    "crazy", "cream", "creek", "crepe", "crest", "crisp", "croak", "crook",
    "crops", "cross", "crowd", "crown", "crumb", "crush", "crust", "cubic",
    "cumin", "curly", "curry", "curve", "cycle", "daily", "dairy", "daisy",
    "dance", "dandy", "dared", "dates", "dealt", "debut", "decal", "decay",
    "decor", "delta", "dense", "depth", "derby", "diary", "digit", "diner",
    "dingy", "ditch", "diver", "dizzy", "dodge", "doing", "donor", "donut",
    "dough", "dozen", "draft", "drain", "drama", "drank", "drape", "drawn",
    "dream", "dress", "dried", "drift", "drill", "drink", "drive", "drone",
    "drove", "drums", "dryer", "duvet", "dwarf", "dwell", "eager", "eagle",
    "early", "earth", "easel", "eaten", "edges", "eight", "elbow", "elder",
    "elect", "elite", "email", "ember", "empty", "enact", "enjoy", "enter",
    "entry", "equal", "equip", "erase", "error", "essay", "ethic", "event",
    "every", "exact", "exams", "exert", "exile", "exist", "extra", "fable",
    "faced", "facts", "faint", "fairy", "faith", "famed", "fancy", "fauna",
    "favor", "feast", "fence", "ferry", "fetch", "fever", "fiber", "field",
    "fiery", "fifth", "fifty", "fight", "filed", "final", "finch", "first",
    "fixed", "fizzy", "fjord", "flair", "flake", "flaky", "flame", "flare",
    "flash", "flask", "fleet", "flesh", "flick", "fling", "flint", "float",
    "flock", "floor", "flora", "floss", "flour", "flown", "fluff", "fluid",
    "flung", "flush", "flute", "foamy", "focal", "focus", "foggy", "folio",
    "forge", "forte", "forth", "forty", "forum", "found", "frame", "frank",
    "fresh", "fried", "front", "frost", "froze", "fruit", "fudge", "fully",
    "fungi", "funny", "fuzzy", "gauge", "gavel", "gecko", "genre", "ghost",
    "giant", "given", "giver", "glade", "gland", "glare", "glass", "gleam",
    "glide", "globe", "gloss", "glove", "gnome", "going", "goose", "gourd",
    "grace", "grade", "grain", "grand", "grant", "grape", "graph", "grasp",
    "grass", "grave", "gravy", "graze", "great", "green", "greet", "grill",
    "grind", "groan", "grove", "growl", "grown", "guard", "guava", "guess",
    "guest", "guide", "gulch", "gusto", "habit", "happy", "hardy", "hasty",
    "hatch", "haven", "hazel", "heard", "heart", "heavy", "hedge", "hefty",
    "hello", "herbs", "hilly", "hinge", "hippo", "hoist", "holly", "homey",
    "honey", "honor", "horns", "horse", "hotel", "hound", "hours", "house",
    "hover", "human", "humid", "humor", "hurry", "husky", "hutch", "icing",
    "ideal", "image", "imply", "index", "inlet", "inner", "input", "irony",
    "issue", "ivory", "jazzy", "jeans", "jelly", "jewel", "joint", "jolly",
    "joust", "judge", "juice", "juicy", "jumbo", "jumpy", "keeps", "kiosk",
    "kites", "knack", "knead", "kneel", "knelt", "knife", "knock", "known",
    "koala", "label", "labor", "ladle", "lakes", "large", "laser", "latch",
    "later", "laugh", "layer", "leafy", "leapt", "learn", "lease", "leash",
    "least", "leave", "ledge", "legal", "lemon", "level", "lever", "light",
    "lilac", "limbs", "limit", "linen", "liner", "lions", "liter", "lived",
    "liver", "livid", "llama", "lobby", "local", "lodge", "lofty", "logic",
    "loose", "lotus", "loved", "lower", "loyal", "lucid", "lucky", "lumen",
    "lunar", "lunch", "lyric", "macaw", "madly", "magic", "maize", "major",
    "maker", "mango", "maple", "march", "marsh", "match", "maybe", "mayor",
    "medal", "media", "melon", "mercy", "merge", "merit", "merry", "metal",
    "meter", "micro", "midst", "might", "milky", "mimic", "minor", "minty",
    "minus", "mirth", "mixer", "mocha", "model", "moist", "molar", "money",
    "month", "moose", "moral", "mossy", "motel", "motor", "mound", "mount",
    "mourn", "mouse", "mouth", "mover", "movie", "mower", "mural", "music",
    "musty", "naval", "nerve", "never", "newer", "newly", "niche", "night",
    "ninth", "noble", "noise", "nomad", "north", "notch", "noted", "novel",
    "nurse", "nutty", "nylon", "oasis", "occur", "ocean", "offer", "often",
    "olden", "older", "olive", "onion", "opera", "opted", "optic", "orbit",
    "order", "otter", "ought", "ounce", "outer", "owing", "owner", "oxide",
    "ozone", "paddy", "pages", "paint", "pairs", "palms", "panda", "panel",
    "pansy", "pants", "paper", "parka", "parts", "party", "pasta", "paste",
    "patch", "patio", "pause", "peace", "peach", "pearl", "pecan", "pedal",
    "penny", "perch", "pesto", "petal", "phase", "phone", "photo", "piano",
    "piece", "pilot", "pinch", "pines", "pinto", "pivot", "pixel", "pizza",
    "place", "plaid", "plain", "plane", "plant", "plate", "plaza", "plead",
    "pluck", "plumb", "plums", "plush", "poems", "point", "polar", "porch",
    "poser", "pouch", "pound", "power", "press", "price", "pride", "prime",
    "print", "prior", "prism", "prize", "probe", "prone", "proof", "prose",
    "proud", "prove", "prune", "pulse", "punch", "pupil", "puppy", "purse",
    "quack", "quail", "qualm", "quart", "queen", "query", "quest", "queue",
    "quick", "quiet", "quill", "quilt", "quirk", "quota", "quote", "radar",
    "radio", "rainy", "raise", "rally", "ranch", "range", "rapid", "ratio",
    "raven", "rayon", "reach", "react", "reads", "ready", "realm", "rebel",
    "refer", "reign", "relax", "relay", "renew", "repay", "reply", "rerun",
    "reset", "resin", "retro", "rhyme", "rider", "ridge", "rifle", "right",
    "rigid", "rinse", "ripen", "risen", "risky", "rival", "river", "roast",
    "robin", "robot", "rocky", "rodeo", "roomy", "roost", "roots", "rotor",
    "rouge", "rough", "round", "route", "rover", "royal", "ruddy", "ruler",
    "rumor", "rural", "rusty", "sadly", "safer", "salad", "salsa", "salty",
    "sandy", "sauce", "sauna", "savor", "scale", "scalp", "scarf", "scene",
    "scent", "scoop", "scope", "score", "scout", "scrap", "scrub", "seals",
    "seats", "sedan", "seeds", "serve", "seven", "sewer", "shade", "shady",
    "shaft", "shake", "shaky", "shale", "shall", "shape", "share", "sharp",
    "shave", "shawl", "shear", "sheen", "sheep", "sheet", "shelf", "shell",
    "shift", "shine", "shiny", "shirt", "shoal", "shock", "shone", "shore",
    "short", "shout", "shown", "showy", "shrub", "shrug", "sight", "silky",
    "since", "siren", "sixth", "sixty", "skate", "skies", "skill", "skirt",
    "skunk", "slant", "slate", "sleek", "sleep", "sleet", "slept", "slice",
    "slide", "sling", "slope", "sloth", "small", "smart", "smash", "smell",
    "smile", "smoke", "snack", "snail", "snake", "sneak", "snore", "snout",
    "snowy", "sober", "socks", "soggy", "solar", "solid", "solve", "sonar",
    "sonic", "sorry", "sound", "south", "space", "spade", "spare", "spark",
    "spawn", "speak", "spear", "speed", "spell", "spend", "spent", "spice",
    "spill", "spine", "spiny", "spire", "splat", "split", "spoke", "spool",
    "spoon", "sport", "spout", "spray", "spree", "sprig", "spurt", "squad",
    "squid", "stack", "staff", "stage", "stair", "stake", "stale", "stalk",
    "stall", "stamp", "stand", "stare", "start", "state", "stays", "steak",
    "steam", "steel", "steep", "steer", "stems", "steps", "stern", "stick",
    "stiff", "still", "sting", "stint", "stock", "stole", "stomp", "stone",
    "stood", "stool", "stoop", "store", "stork", "storm", "story", "stout",
    "stove", "strap", "straw", "stray", "strip", "strut", "stuck", "study",
    "stump", "stunt", "style", "suave", "suede", "sugar", "suite", "sunny",
    "super", "surge", "sushi", "swamp", "swans", "swarm", "sways", "sweet",
    "swept", "swift", "swing", "swirl", "swoon", "sword", "syrup", "tabby",
    "table", "taken", "tally", "tango", "tangy", "taste", "tasty", "teach",
    "teams", "tempo", "tends", "tenor", "tense", "tenth", "thank", "theme",
    "there", "thick", "thief", "thigh", "thing", "think", "third", "thorn",
    "those", "three", "threw", "throw", "thumb", "thyme", "tidal", "tiger",
    "tiled", "timer", "tired", "title", "toast", "today", "token", "tonic",
    "tooth", "topic", "torch", "total", "touch", "tough", "towel", "tower",
    "toxic", "trace", "track", "trade", "trail", "train", "trait", "tramp",
    "trees", "trend", "trial", "tribe", "trick", "tried", "trout", "truce",
    "truck", "truly", "trunk", "trust", "truth", "tulip", "tumor", "tuned",
    "tunic", "turbo", "tutor", "twice", "twigs", "twine", "twirl", "twist",
    "udder", "ultra", "uncle", "under", "undue", "union", "unite", "unity",
    "until", "upper", "upset", "urban", "usage", "usher", "usual", "utter",
    "vague", "valid", "valor", "value", "valve", "vapor", "vault", "vegan",
    "venue", "verse", "video", "vigor", "vinyl", "viola", "viper", "visit",
    "vista", "vital", "vivid", "vocal", "vogue", "voice", "volts", "voter",
    "vouch", "vowel", "wafer", "wagon", "waist", "waltz", "warms", "waste",
    "watch", "water", "waves", "weary", "weave", "wedge", "weeds", "weeks",
    "weigh", "weird", "wells", "whale", "wharf", "wheat", "wheel", "where",
    "which", "while", "whirl", "whisk", "white", "whole", "whose", "widen",
    "wider", "width", "wield", "windy", "wiper", "wired", "wiser", "witty",
    "woman", "women", "woods", "woody", "wooly", "words", "works", "world",
    "worse", "worth", "would", "woven", "wrath", "wreck", "wring", "wrist",
    "write", "wrote", "wrung", "yacht", "yards", "yarns", "yearn", "yeast",
    "yield", "young", "yours", "youth", "yummy", "zebra", "zesty", "zonal",
    "zooms",
];

#[cfg(test)]
mod tests {
    use super::*;

    /// The lists prove their own rules: five lowercase ASCII letters per word,
    /// no duplicates, in both lists — and every secret is an accepted guess.
    /// Lengths are measured off each word, never assumed.
    #[test]
    fn the_list_is_sound() {
        for (name, list) in [("WORDS", WORDS), ("ACCEPTED", ACCEPTED)] {
            assert!(!list.is_empty(), "{name} is empty");
            for word in list {
                assert_eq!(word.len(), 5, "not five letters in {name}: {word}");
                for c in word.chars() {
                    assert!(c.is_ascii_lowercase(), "not a-z in {name}: {word}");
                }
            }
            for (i, a) in list.iter().enumerate() {
                for b in &list[i + 1..] {
                    assert_ne!(a, b, "duplicate word in {name}: {a}");
                }
            }
        }
        for secret in WORDS {
            assert!(
                ACCEPTED.contains(secret),
                "a secret the game would refuse as a guess: {secret}"
            );
        }
    }
}
