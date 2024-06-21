use ratatui::prelude::*;

pub struct KeyboardLayout {
    pub rows: [[String; 10]; 3],
}

#[derive(Clone, Debug)]
pub struct Split {
    time: u128,
    hits: u64,
    misses: u64,
}

impl Split {
    pub fn new(hits: u64, misses: u64, time: u128) -> Split {
        Split { hits, misses, time }
    }

    fn words(&self, chars: u64) -> f64 {
        chars as f64 / 5.0
    }

    pub fn minutes(&self) -> f64 {
        self.time() / 60.0
    }

    pub fn time(&self) -> f64 {
        self.time as f64 / 1e9
    }

    pub fn wpm(&self) -> f64 {
        self.words(self.hits) / self.minutes()
    }

    pub fn raw(&self) -> f64 {
        self.words(self.hits + self.misses) / self.minutes()
    }
}

#[derive(Clone, Debug)]
pub struct Race {
    pub length: u64,
    misses: u64,
    pub time: u128,
    pub splits: Vec<Split>,
}

impl Race {
    pub fn new(length: u64, misses: u64, time: u128, splits: Vec<Split>) -> Race {
        Race { length, misses, time, splits } 
    }

    pub fn time(&self) -> f64 {
        self.time as f64 / 1e9
    }

    pub fn minutes(&self) -> f64 {
        self.time() / 60.0
    }

    pub fn words(&self, chars: u64) -> f64 {
        chars as f64 / 5.0
    }

    pub fn raw(&self) -> f64 {
        self.words(self.length + self.misses) / self.time()
    }
    
    pub fn wpm(&self) -> f64 {
        self.words(self.length) / self.minutes()
    }

    pub fn wpm_data(&self) -> Vec<(f64, f64)> {
        let mut splits: Vec<(f64, f64)> = self.splits
            .iter()
            .map(|split| (split.time(), split.wpm()))
            .collect();
        splits.push((self.time(), self.wpm()));
        splits
    }

    pub fn raw_data(&self) -> Vec<(f64, f64)> {
        let mut splits: Vec<(f64, f64)> = self.splits
            .iter()
            .map(|split| (split.time(), split.raw()))
            .collect();
        splits.push((self.time(), self.raw()));
        return splits
    }

    pub fn accuracy(&self) -> f64 {
        self.length as f64 / (self.length + self.misses) as f64 * 100.0
    }
}

pub struct Quote {
    name: String,
    text: String,
}

impl Quote {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    
    pub fn new(name: String, text: String) -> Quote {
        Quote { name, text }
    }
}

pub enum Keystroke {
    Wrong,
    Correct,
    Quit,
    Invalid,
}

pub const CORRECT: Color = Color::Rgb(80, 200, 120);
pub const INCORRECT: Color = Color::Red;
pub const TITLE: Color = Color::Rgb(100, 149, 237);

pub const QUOTES: [(&str, &str); 10] = [
        ("Raising Smart Kids for Dummies", "The sooner your kids appreciate the value of work, the more successful they will be. Work is part of life. You work to earn money, put food on the table, and keep your homes orderly and clean. For your kids, work involves schoolwork, homework, and teamwork at home and in the community."),
        ("The Empire Strikes Back", "If only you'd attached my legs, I wouldn't be in this ridiculous position. Now remember, Chewbacca, you have a responsibility to me, so don't do anything foolish!"),
        ("Dictionary", "feel number do last public life follow do this even both need day own possible like right come place during real child line face as work"),
        ("The Legend of Zelda: The Wind Waker", "In order to return the power to repel evil to your sword, you must find another to take my stead in this temple and ask the gods for their assistance. You must find the one who carries on my bloodline... The one who holds this sacred instrument."),
        ("The Unbearable Lightness of Being", "It may seem quite novelistic to you, and I am willing to agree, but only on the condition that you refrain from reading such notions as 'fictive', 'fabricated', and 'untrue to life' in the word 'novelistic'. Because human lives are composed in precisely such a fashion."),
        ("Her", "Women like her are only hard to love by men who believe love is just a word."),
        ("Lock, Stock, and Two Smoking Barrels", "What else do I get with it? - You get a gold-plated Rolls Royce, as long as you pay for it. - Don't know, Tom, seems expensive. - Seems... well this seems to be a waste of my time. That is nine hundred nicker in any shop you're lucky enough to find one in, and you're complaining about two hundred? What school of finance did you study? It's a deal, it's a steal, it's the sale of the beeping century! In fact, beep it Nick, I think I'll keep it!"),
        ("Through the Fire and Flames", "We feel the pain of a lifetime lost in a thousand days."),
        ("Seven Seas", "Burning my bridges and smashing my mirrors, turning to see if you're cowardly. Burning the witches with modern religions, you'll strike the matches and shower me. In water games washing the rocks below. Taught and tamed in time with tear flow."),
        ("Avengers: Infinity War", "With all six stones, I could simply snap my fingers, they would all cease to exist and I call that... mercy. And then what? I finally rest, and watch the sun rise on a grateful universe. The hardest choices require the strongest wills.")
    ];


pub const ASCII_ART_1: &str = r#"
$$$$$$$$\                                
\__$$  __|                               
   $$ | $$$$$$\   $$$$$$\  $$$$$$\$$$$\  
   $$ |$$  __$$\ $$  __$$\ $$  _$$  _$$\ 
   $$ |$$$$$$$$ |$$ |  \__|$$ / $$ / $$ |
   $$ |$$   ____|$$ |      $$ | $$ | $$ |
   $$ |\$$$$$$$\ $$ |      $$ | $$ | $$ |
   \__| \_______|\__|      \__| \__| \__|
"#;

pub const ASCII_ART_2: &str = r#"
$$$$$$$$\                                      
\__$$  __|                                     
   $$ |$$\   $$\  $$$$$$\   $$$$$$\   $$$$$$\  
   $$ |$$ |  $$ |$$  __$$\ $$  __$$\ $$  __$$\ 
   $$ |$$ |  $$ |$$ /  $$ |$$$$$$$$ |$$ |  \__|
   $$ |$$ |  $$ |$$ |  $$ |$$   ____|$$ |      
   $$ |\$$$$$$$ |$$$$$$$  |\$$$$$$$\ $$ |      
   \__| \____$$ |$$  ____/  \_______|\__|      
       $$\   $$ |$$ |                          
       \$$$$$$  |$$ |                          
        \______/ \__|                          
"#;

