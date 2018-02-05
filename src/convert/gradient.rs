// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use svgdom;

use dom;

use short::{
    AId,
    EId,
};

use traits::{
    GetValue,
};


pub fn convert_linear(
    node: &svgdom::Node,
    doc: &mut dom::Document,
) {
    let ref attrs = node.attributes();

    doc.append_node(dom::DEFS_DEPTH, dom::NodeKind::LinearGradient(dom::LinearGradient {
        id: node.id().clone(),
        x1: attrs.get_number(AId::X1).unwrap_or(0.0),
        y1: attrs.get_number(AId::Y1).unwrap_or(0.0),
        x2: attrs.get_number(AId::X2).unwrap_or(1.0),
        y2: attrs.get_number(AId::Y2).unwrap_or(0.0),
        d: dom::BaseGradient {
            units: super::convert_element_units(attrs, AId::GradientUnits),
            transform: attrs.get_transform(AId::GradientTransform).unwrap_or_default(),
            spread_method: convert_spread_method(&attrs),
        }
    }));

    convert_stops(node, doc);
}

pub fn convert_radial(
    node: &svgdom::Node,
    doc: &mut dom::Document,
) {
    let ref attrs = node.attributes();

    doc.append_node(dom::DEFS_DEPTH, dom::NodeKind::RadialGradient(dom::RadialGradient {
        id: node.id().clone(),
        cx: attrs.get_number(AId::Cx).unwrap_or(0.5),
        cy: attrs.get_number(AId::Cy).unwrap_or(0.5),
        r:  attrs.get_number(AId::R).unwrap_or(0.5),
        fx: attrs.get_number(AId::Fx).unwrap_or(0.5),
        fy: attrs.get_number(AId::Fy).unwrap_or(0.5),
        d: dom::BaseGradient {
            units: super::convert_element_units(attrs, AId::GradientUnits),
            transform: attrs.get_transform(AId::GradientTransform).unwrap_or_default(),
            spread_method: convert_spread_method(&attrs),
        }
    }));

    convert_stops(node, doc);
}

fn convert_spread_method(
    attrs: &svgdom::Attributes
) -> dom::SpreadMethod {
    let av = attrs.get_predef(AId::SpreadMethod).unwrap_or(svgdom::ValueId::Pad);

    match av {
        svgdom::ValueId::Pad => dom::SpreadMethod::Pad,
        svgdom::ValueId::Reflect => dom::SpreadMethod::Reflect,
        svgdom::ValueId::Repeat => dom::SpreadMethod::Repeat,
        _ => dom::SpreadMethod::Pad,
    }
}

fn convert_stops(
    node: &svgdom::Node,
    doc: &mut dom::Document,
) {
    for s in node.children() {
        if !s.is_tag_name(EId::Stop) {
            debug!("Invalid gradient child: '{:?}'.", s.tag_id().unwrap());
            continue;
        }

        let attrs = s.attributes();

        // Tested by:
        // - pservers-grad-18-b.svg
        let offset = attrs.get_number(AId::Offset).unwrap_or(0.0);
        let color = attrs.get_color(AId::StopColor).unwrap_or(svgdom::Color::new(0, 0, 0));
        let opacity = attrs.get_number(AId::StopOpacity).unwrap_or(1.0);

        doc.append_node(dom::DEFS_DEPTH + 1, dom::NodeKind::Stop(dom::Stop {
            offset,
            color,
            opacity,
        }));
    }
}