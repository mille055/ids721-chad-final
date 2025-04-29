use projectf::dictionary::SimpleDictionary;

#[test]
fn test_lookup_terms_from_json() {
    let dict = SimpleDictionary::load_from_json("data/medical_terms.json");

    let jejunum_def = dict.lookup("Jejunum");
    assert!(jejunum_def.is_some());
    assert_eq!(
        jejunum_def.unwrap(),
        "The second part of the small intestine, where most nutrient absorption occurs."
    );

    let sigmoid_def = dict.lookup("sigmoid colon");
    assert!(sigmoid_def.is_some());
    assert_eq!(
        sigmoid_def.unwrap(),
        "The part of the large intestine just before the rectum, where waste is stored before being eliminated."
    );

    let rectum_def = dict.lookup("rectum");
    assert!(rectum_def.is_some());
    assert_eq!(
        rectum_def.unwrap(),
        "The final part of the large intestine, where waste is stored before being eliminated."
    );

    let nonexistent = dict.lookup("pancreas juice");
    assert!(nonexistent.is_none());
}

#[test]
fn test_highlight_medical_terms_from_json() {
    let dict = SimpleDictionary::load_from_json("data/medical_terms.json");

    let input = "The jejunum and sigmoid colon were evaluated.";
    let highlighted = dict.highlight_medical_terms(input);

    assert!(
        highlighted.contains("jejunum") && highlighted.contains("sigmoid colon"),
        "Expected terms not found in highlighted output"
    );

    // Check that span is inserted
    assert!(
        highlighted.contains("style=\"text-decoration: underline dotted;"),
        "Highlighting span with style not found"
    );

    // Check newlines are converted (even though this input has none)
    assert!(!highlighted.contains('\n'));
}
