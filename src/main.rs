use resvg::usvg::Transform;
use resvg::{tiny_skia, usvg};

fn main() {
    let tree = {
        let mut opt = usvg::Options::default();
        opt.resources_dir = std::fs::canonicalize("africa.svg")
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));

        opt.fontdb_mut().load_system_fonts();

        let svg_data = std::fs::read("africa.svg").unwrap();
        usvg::Tree::from_data(&svg_data, &opt).unwrap()
    };

    let africa = tree.node_by_id("africa").unwrap();
    if let usvg::Node::Group(ref group) = africa {
        let country = group.children().get(20).unwrap();
        let bbox = country.abs_layer_bounding_box().unwrap();
        let width = bbox.width().ceil() as u32;
        let height = bbox.height().ceil() as u32;
        println!("width: {}, height: {}", width, height);

        let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
        let transform = Transform::from_translate(bbox.x(), bbox.y());
        resvg::render_node(country, transform, &mut pixmap.as_mut());
        pixmap.save_png("country.png").unwrap();
    }
}
