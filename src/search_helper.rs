use eframe::egui::{Align, Color32, FontSelection, Label, RichText, Style, Ui, text::LayoutJob};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SearchConfig{
    priority_between_spaces:f32,
    priority_after_space:f32,
    priority_anywhere_whole:f32,
}
impl Default for SearchConfig{
    fn default() -> Self {
        Self { priority_between_spaces: 25.0, priority_after_space: 15.0, priority_anywhere_whole: 0.0 }
    }
}
pub fn search(query: &String, list: &Vec<String>, config:&SearchConfig) -> Vec<(f32, usize, Vec<usize>)> {
    if query.len()==0{
        return (0..list.len()).map(|x| (0.0, x, Vec::new())).collect::<Vec<_>>();
    }
    let lower_case = query.to_lowercase();
    let non_found_ids = (0..list.len()).collect::<Vec<usize>>();
    let mut s_anywhere_whole = non_found_ids
        .clone()
        .into_iter()
        .filter_map(|i| {
            list[i].to_lowercase().find(&lower_case).map(|h| {// id of start of query in lower case list entry
                let (_, lookup) = to_lowercase_lookup(&list[i]);
                let h2 = lookup.iter().position(|x| x==&h).unwrap_or_else(|| panic!("{h}={}", list[i]));
                let h3 = lookup.iter().position(|x| x==&(h + lower_case.len())).unwrap_or_else(|| panic!("{h}={}", list[i]));
                (config.priority_anywhere_whole, i, vec![h2, h3])// id of start/end of query in uppercase list entry
            })
        })
        .collect::<Vec<(f32, usize, Vec<usize>)>>();
    let mut non_found_ids=s_anywhere_whole.iter().map(|(_,i, _)| *i).collect::<Vec<_>>();
    let seperator_chars = vec![' ', '.', '-', '_', ';', ',']
        .drain(..)
        .filter(|c| !lower_case.contains(&c.to_lowercase().to_string()))
        .collect::<Vec<char>>();
    let mut s_between_spaces = non_found_ids
        .clone()
        .into_iter()
        .filter_map(|i| {
            split_multiple(&seperator_chars, list[i].clone())
                .iter()
                .find_map(|s| {
                    (s.1.to_lowercase() == *lower_case).then(|| (config.priority_between_spaces,i, vec![s.0, s.0 + s.1.len()]))
                })
        })
        .collect::<Vec<(f32, usize, Vec<usize>)>>();
    remove_found(&mut non_found_ids, &mut s_between_spaces);
    s_between_spaces.sort_by_key(|(_, _, h)| h[0]);
    let mut s_after_space = non_found_ids
        .clone()
        .into_iter()
        .filter_map(|i| {
            split_multiple(&seperator_chars, list[i].clone())
                .iter()
                .find_map(|s| {
                    (s.1.to_lowercase().starts_with(&lower_case)).then(|| {
                        let (_, lookup) = to_lowercase_lookup(&s.1);
                        let h3 = lookup[lower_case.len()];
                        (config.priority_after_space, i, vec![s.0, s.0 + h3])
                    })
                })
        })
        .collect::<Vec<(f32, usize, Vec<usize>)>>();
    remove_found(&mut non_found_ids, &mut s_after_space);
    s_after_space.sort_by_key(|(_, _, h)| h[0]);
    s_anywhere_whole.sort_by_key(|(_,_, h)| h[0]);
    let mut temp=s_between_spaces
        .drain(..)
        .chain(s_after_space.drain(..)).collect::<Vec<_>>();
    let mut s_anywhere_whole=s_anywhere_whole.drain(..).filter(|x| temp.iter().find(|y| y.1==x.1).is_none()).collect::<Vec<_>>();
        temp.drain(..).chain(s_anywhere_whole.drain(..))
        .collect()
}
fn remove_found(non_found_ids: &mut Vec<usize>, s_at_start: &mut Vec<(f32, usize, Vec<usize>)>) {
    *non_found_ids = non_found_ids
        .drain(..)
        .filter(|i| s_at_start.binary_search_by_key(i, |(_, a, _)| *a).is_err())
        .collect::<Vec<usize>>();
}
pub fn split_multiple(seperator_chars: &Vec<char>, mut string: String) -> Vec<(usize, String)> {
    let binding = string.clone();
    let mut t = binding
        .match_indices(|c| seperator_chars.contains(&c))
        .map(|(i, _)| i)
        .rev()
        .map(|i| {
            let t = string.split_off(i + 1);
            string.truncate(string.len() - 1);
            (string.len() + 1, t)
        })
        .collect::<Vec<(usize, String)>>();
    t.push((0, string));
    t.reverse();
    t
}
fn to_lowercase_lookup(s: &String) -> (String, Vec<usize>) {
    let mut string = String::new();
    let mut indices = vec![0];
    for c in s.chars() {
        let prev = *indices.last().unwrap();
        for _ in 0..(c.len_utf8() - 1) {
            indices.push(usize::MAX);
        }
        let lowercase = c.to_lowercase();
        indices.push(prev + lowercase.to_string().len());
        string.push(c);
    }
    (string, indices)
}
pub fn mark_text(s: String, mark: &Vec<usize>, ui: &mut Ui) {
    let style = Style::default();
    let mut text = LayoutJob::default();
    let mut last = 0;
    let mut marked = false;
    for i in mark.iter().chain(std::iter::once(&s.len())) {
        let curtxt = s[last..*i].to_string();
        if !marked {
            RichText::new(curtxt)
                .color(Color32::from_rgb(255, 255, 255))
                .append_to(&mut text, &style, FontSelection::Default, Align::Center);
        } else {
            RichText::new(curtxt)
                .color(Color32::from_rgb(0, 255, 255))
                .underline()
                .append_to(&mut text, &style, FontSelection::Default, Align::Center);
        }
        last = *i;
        marked = !marked;
    }
    ui.add(Label::new(text).wrap());
}

/*
todo:
for search term cinnamonn:
cinnamon2d

for search term cinnamon screensaver:
cinnamon-screensaver



search for exact string at start, after+bevore/after whitespace/punctuation/...
recursive call for whitespace-split string? How do I sort that?
*/
