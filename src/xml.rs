use walkdir::WalkDir;
use xml_builder::{XMLBuilder, XMLElement, XMLVersion};

fn traverse_assets(parent: &mut XMLElement, path: &str) -> Result<(), walkdir::Error> {
    let wd = WalkDir::new(path)
        .max_depth(1)
        .follow_links(false)
        .sort_by(|a, b| {
            let is_a_dir = a.metadata().unwrap().is_dir();
            let is_b_dir = b.metadata().unwrap().is_dir();
            if is_a_dir && !is_b_dir {
                return std::cmp::Ordering::Less;
            } else if !is_a_dir && is_b_dir {
                return std::cmp::Ordering::Greater;
            }
            a.file_name().cmp(b.file_name())
        });
    for e in wd {
        let entry = e?;
        if entry.path() == path {
            continue;
        }
        let child = if entry.metadata()?.is_dir() {
            let mut dir = XMLElement::new("dir");
            dir.add_attribute("name", &entry.file_name().to_str().unwrap().to_uppercase());
            traverse_assets(&mut dir, entry.path().as_os_str().to_str().unwrap())?;
            dir
        } else {
            let mut file = XMLElement::new("file");
            file.add_attribute("name", &entry.file_name().to_str().unwrap().to_uppercase());
            file.add_attribute("type", "data");
            file.add_attribute("source", entry.path().to_str().unwrap());
            file
        };
        // Safe to ignore error as we are not adding to a text element.
        _ = parent.add_child(child);
    }
    Ok(())
}

pub fn generate_raw_assets_xml(
    path: &str,
    image_name: &str,
    system_id: &str,
    application_id: &str,
) -> Result<Vec<u8>, walkdir::Error> {
    let mut xml = XMLBuilder::new()
        .version(XMLVersion::XML1_1)
        .encoding("UTF-8".into())
        .build();

    let mut iso_project = XMLElement::new("iso_project");
    iso_project.add_attribute("image_name", &format!("{}.bin", image_name));
    iso_project.add_attribute("cue_sheet", &format!("{}.cue", image_name));

    let mut track = XMLElement::new("track");
    track.add_attribute("type", "data");

    let mut identifiers = XMLElement::new("identifiers");
    identifiers.add_attribute("system", system_id);
    identifiers.add_attribute("application", application_id);

    let mut directory_tree = XMLElement::new("directory_tree");
    traverse_assets(&mut directory_tree, path)?;

    // Safe to ignore all the errors, as the error that could be raised is if adding children to a text element.
    _ = track.add_child(identifiers);
    _ = track.add_child(directory_tree);
    _ = iso_project.add_child(track);

    xml.set_root_element(iso_project);
    let mut v = Vec::new();
    xml.generate(&mut v).unwrap();

    Ok(v)
}
