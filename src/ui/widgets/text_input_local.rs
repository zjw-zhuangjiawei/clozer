use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Value {
    graphemes: Vec<String>,
}

impl Value {
    pub fn new(string: &str) -> Self {
        let graphemes = UnicodeSegmentation::graphemes(string, true)
            .map(String::from)
            .collect();

        Self { graphemes }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

    pub fn previous_start_of_word(&self, index: usize) -> usize {
        let previous_string = &self.graphemes[..index.min(self.graphemes.len())].concat();

        UnicodeSegmentation::split_word_bound_indices(previous_string as &str)
            .rfind(|(_, word)| !word.trim_start().is_empty())
            .map(|(i, previous_word)| {
                index
                    - UnicodeSegmentation::graphemes(previous_word, true).count()
                    - UnicodeSegmentation::graphemes(
                        &previous_string[i + previous_word.len()..] as &str,
                        true,
                    )
                    .count()
            })
            .unwrap_or(0)
    }

    pub fn next_end_of_word(&self, index: usize) -> usize {
        let next_string = &self.graphemes[index..].concat();

        UnicodeSegmentation::split_word_bound_indices(next_string as &str)
            .find(|(_, word)| !word.trim_start().is_empty())
            .map(|(i, next_word)| {
                index
                    + UnicodeSegmentation::graphemes(next_word, true).count()
                    + UnicodeSegmentation::graphemes(&next_string[..i] as &str, true).count()
            })
            .unwrap_or(self.len())
    }

    pub fn select(&self, start: usize, end: usize) -> Self {
        let graphemes = self.graphemes[start.min(self.len())..end.min(self.len())].to_vec();
        Self { graphemes }
    }

    #[allow(dead_code)]
    pub fn until(&self, index: usize) -> Self {
        let graphemes = self.graphemes[..index.min(self.len())].to_vec();
        Self { graphemes }
    }

    pub fn insert(&mut self, index: usize, c: char) {
        self.graphemes.insert(index, c.to_string());

        self.graphemes = UnicodeSegmentation::graphemes(&self.to_string() as &str, true)
            .map(String::from)
            .collect();
    }

    pub fn insert_many(&mut self, index: usize, mut value: Value) {
        let _ = self
            .graphemes
            .splice(index..index, value.graphemes.drain(..));
    }

    pub fn remove(&mut self, index: usize) {
        let _ = self.graphemes.remove(index);
    }

    pub fn remove_many(&mut self, start: usize, end: usize) {
        let _ = self.graphemes.splice(start..end, std::iter::empty());
    }

    pub fn secure(&self) -> Self {
        Self {
            graphemes: std::iter::repeat_n(String::from("•"), self.graphemes.len()).collect(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.graphemes.concat())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Index(usize),
    Selection { start: usize, end: usize },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cursor {
    state: State,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            state: State::Index(0),
        }
    }
}

impl Cursor {
    pub fn state(&self, value: &Value) -> State {
        match self.state {
            State::Index(index) => State::Index(index.min(value.len())),
            State::Selection { start, end } => {
                let start = start.min(value.len());
                let end = end.min(value.len());

                if start == end {
                    State::Index(start)
                } else {
                    State::Selection { start, end }
                }
            }
        }
    }

    pub fn selection(&self, value: &Value) -> Option<(usize, usize)> {
        match self.state(value) {
            State::Selection { start, end } => Some((start.min(end), start.max(end))),
            State::Index(_) => None,
        }
    }

    pub fn move_to(&mut self, position: usize) {
        self.state = State::Index(position);
    }

    pub fn move_right(&mut self, value: &Value) {
        self.move_right_by_amount(value, 1);
    }

    pub fn move_right_by_amount(&mut self, value: &Value, amount: usize) {
        match self.state(value) {
            State::Index(index) => {
                self.move_to(index.saturating_add(amount).min(value.len()));
            }
            State::Selection { start, end } => self.move_to(end.max(start)),
        }
    }

    pub fn move_right_by_words(&mut self, value: &Value) {
        self.move_to(value.next_end_of_word(self.right(value)));
    }

    pub fn move_left(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index > 0 => self.move_to(index - 1),
            State::Selection { start, end } => self.move_to(start.min(end)),
            State::Index(_) => self.move_to(0),
        }
    }

    pub fn move_left_by_words(&mut self, value: &Value) {
        self.move_to(value.previous_start_of_word(self.left(value)));
    }

    pub fn select_range(&mut self, start: usize, end: usize) {
        if start == end {
            self.state = State::Index(start);
        } else {
            self.state = State::Selection { start, end };
        }
    }

    pub fn select_left(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index > 0 => {
                self.select_range(index, index - 1);
            }
            State::Selection { start, end } if end > 0 => {
                self.select_range(start, end - 1);
            }
            _ => {}
        }
    }

    pub fn select_right(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index < value.len() => {
                self.select_range(index, index + 1);
            }
            State::Selection { start, end } if end < value.len() => {
                self.select_range(start, end + 1);
            }
            _ => {}
        }
    }

    pub fn select_left_by_words(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) => {
                self.select_range(index, value.previous_start_of_word(index));
            }
            State::Selection { start, end } => {
                self.select_range(start, value.previous_start_of_word(end));
            }
        }
    }

    pub fn select_right_by_words(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) => {
                self.select_range(index, value.next_end_of_word(index));
            }
            State::Selection { start, end } => {
                self.select_range(start, value.next_end_of_word(end));
            }
        }
    }

    pub fn select_all(&mut self, value: &Value) {
        self.select_range(0, value.len());
    }

    pub fn start(&self, value: &Value) -> usize {
        let start = match self.state {
            State::Index(index) => index,
            State::Selection { start, .. } => start,
        };

        start.min(value.len())
    }

    pub fn end(&self, value: &Value) -> usize {
        let end = match self.state {
            State::Index(index) => index,
            State::Selection { end, .. } => end,
        };

        end.min(value.len())
    }

    fn left(&self, value: &Value) -> usize {
        match self.state(value) {
            State::Index(index) => index,
            State::Selection { start, end } => start.min(end),
        }
    }

    fn right(&self, value: &Value) -> usize {
        match self.state(value) {
            State::Index(index) => index,
            State::Selection { start, end } => start.max(end),
        }
    }
}

pub struct Editor<'a> {
    value: &'a mut Value,
    cursor: &'a mut Cursor,
}

impl<'a> Editor<'a> {
    pub fn new(value: &'a mut Value, cursor: &'a mut Cursor) -> Editor<'a> {
        Editor { value, cursor }
    }

    pub fn contents(&self) -> String {
        self.value.to_string()
    }

    pub fn insert(&mut self, character: char) {
        if let Some((left, right)) = self.cursor.selection(self.value) {
            self.cursor.move_left(self.value);
            self.value.remove_many(left, right);
        }

        self.value.insert(self.cursor.end(self.value), character);
        self.cursor.move_right(self.value);
    }

    pub fn paste(&mut self, content: Value) {
        let length = content.len();
        if let Some((left, right)) = self.cursor.selection(self.value) {
            self.cursor.move_left(self.value);
            self.value.remove_many(left, right);
        }

        self.value.insert_many(self.cursor.end(self.value), content);

        self.cursor.move_right_by_amount(self.value, length);
    }

    pub fn backspace(&mut self) {
        match self.cursor.selection(self.value) {
            Some((start, end)) => {
                self.cursor.move_left(self.value);
                self.value.remove_many(start, end);
            }
            None => {
                let start = self.cursor.start(self.value);

                if start > 0 {
                    self.cursor.move_left(self.value);
                    self.value.remove(start - 1);
                }
            }
        }
    }

    pub fn delete(&mut self) {
        match self.cursor.selection(self.value) {
            Some(_) => {
                self.backspace();
            }
            None => {
                let end = self.cursor.end(self.value);

                if end < self.value.len() {
                    self.value.remove(end);
                }
            }
        }
    }
}
