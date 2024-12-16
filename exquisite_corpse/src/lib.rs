pub fn add_line_and_make_response(line: &str, text: &mut String) -> String {
    // Ajoute la ligne au texte et renvoie le texte
    if !text.is_empty() {
        // Ajoute un espace si le texte n'est pas vide
        text.push(' ');
    }
    text.push_str(line);
    text.to_string()
}