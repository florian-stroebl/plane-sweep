// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)

use common::{
    AlgoStepIdx, AlgoSteps,
    intersection::Intersections,
    math::A,
    segment::{SegmentIdx, Segments},
    ui::{MyWidget, WidgetName},
};
use eframe::egui::RichText;

use crate::{Step, StepType};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeView;

impl WidgetName for CodeView {
    const NAME: &'static str = "Code";
    const NAME_LONG: &'static str = "Code Viewer";
}
#[derive(Debug, Clone)]
pub struct CodeViewState<'a, 'b, 'c, T: A> {
    pub step: AlgoStepIdx,
    pub steps: &'a AlgoSteps<Step<T>>,
    pub segments: &'b Segments<T>,
    pub intersections: &'c Intersections<T>,
}

impl<'a, 'b, 'c, T: A> MyWidget<CodeViewState<'a, 'b, 'c, T>> for CodeView {
    #[allow(clippy::too_many_lines)]
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<CodeViewState<'a, 'b, 'c, T>>) {
        let CodeViewState {
            step,
            steps,
            segments,
            intersections,
        } = state.into();
        let s = &steps[step];

        if s.typ.is_init() {
            ui.label(RichText::new("Not yet started").heading().underline());
        }

        let text = RichText::new("Find Intersections").heading();
        ui.label(if s.typ.is_find_intersections() {
            text.underline()
        } else {
            text
        });

        let text = RichText::new("  Initialize an empty event queue Q");
        ui.label(if matches!(s.typ, crate::StepType::StartInitQ) {
            text.underline()
        } else {
            text
        });
        if let StepType::InitQ { segment } = s.typ {
            let seg = &segments[segment];

            let text = RichText::new(format!(
                "  Inserting segment s{} with events (y,x,segment) ({:.2}, {:.2}, s{}) and ({:.2}, {:.2}, ())",
                seg.id, seg.upper.y, seg.upper.x, seg.id, seg.lower.y, seg.lower.x
            )).underline();
            ui.label(text);
        } else {
            ui.label("  Insert segment endpoints into Q");
        }

        let text = RichText::new(" Initialize an empty status queue T");
        ui.label(if matches!(s.typ, crate::StepType::InitT) {
            text.underline()
        } else {
            text
        });

        if s.typ == StepType::PopQ {
            let event = s.p.as_ref().unwrap();
            let y = &event.y;
            let x = &event.x;
            let seg = format_segment(s.u_p.iter(), segments);

            let text = RichText::new(format!("  while Q is not empty, pop the next event point. The next event is ({y}, {x}, ({seg}))")).underline();
            ui.label(text);
        } else {
            ui.label("  while Q is not empty, pop the next event point.");
        }
        ui.separator();

        let text = RichText::new("Handle Event Point").heading();
        ui.label(if s.typ.is_handle_event_point() {
            text.underline()
        } else {
            text
        });
        let text = RichText::new("  Calculate the Set U(p), C(p), L(p)");
        ui.label(if s.typ == StepType::CalculateSets {
            text.underline()
        } else {
            text
        });
        if let StepType::CalculateUpCpLp { up_cp_lp } = &s.typ {
            let seg = format_segment(up_cp_lp.iter(), segments);
            let text = RichText::new(format!(
                "  Calculate the set U(p) and C(p) and L(p): {seg}"
            ))
            .underline();
            ui.label(text);
        } else {
            ui.label("  Calculate the set U(p) and C(p) and L(p)");
        }
        if let StepType::ReportIntersections { intersection } = s.typ {
            let intersection = intersections[intersection].step();
            ui.label(RichText::new(format!("  If U(p) and C(p) and L(p) >= 2, report an intersection. Adding intersection {intersection}")).underline());
        } else {
            ui.label("  If U(p) and C(p) and L(p) >= 2, report an intersection.");
        }
        let text = RichText::new("  Delete C(p) and L(p) from the status queue");
        ui.label(if s.typ == StepType::DeleteLpCp {
            text.underline()
        } else {
            text
        });
        let text = RichText::new("  Insert U(p) into the status queue");
        ui.label(if s.typ == StepType::InsertUpCp {
            text.underline()
        } else {
            text
        });
        ui.label("  if U(p) and C(p) = empty");

        if let StepType::UpCpEmpty { s_l, s_r } = &s.typ {
            let s_l = format_segment(s_l.iter(), segments);
            let s_r = format_segment(s_r.iter(), segments);

            let text= RichText::new(format!("    then Let s_l and s_r be the left and right neighbors of event p in our StatusQueue. s_l = ({s_l}), s_r = ({s_r})")).underline();
            ui.label(text);
            ui.label(RichText::new("    FindNewEvent(s_l, s_r, p)").underline());
        } else {
            ui.label("    then Let s_l and s_r be the left and right neighbors of event p in our StatusQueue.");
            ui.label("    FindNewEvent(s_l, s_r, p)");
        }
        if let StepType::UpCpNotEmpty {
            s_r,
            s_dash,
            s_dash_dash,
            s_l,
        } = &s.typ
        {
            let s_l = format_segment(s_l.iter(), segments);
            let s_r = format_segment(s_r.iter(), segments);
            let s_dash = s_dash.map(|s_dash| segments[s_dash].id);
            let s_dash_dash = s_dash_dash.map(|s_dash_dash| segments[s_dash_dash].id);
            let text= RichText::new(format!("    else Let s' be the leftmost segment of U(p) and C(p) in the StatusQueue. s' = s{s_dash:?}")).underline();
            ui.label(text);
            let text = RichText::new(format!(
                "    Let s_l be the left neighbor of s' in the StatusQueue. s_l = ({s_l})"
            ))
            .underline();
            ui.label(text);
            ui.label(RichText::new("    FindNewEvent(s_l, s', p)").underline());
            let text= RichText::new(format!("    Let s'' be the rightmost segment of U(p) and C(p) in the StatusQueue. s'' = s{s_dash_dash:?}")).underline();
            ui.label(text);
            let text = RichText::new(format!(
                "    Let s_r be the right neighbor of s'' in the StatusQueue. s_r = ({s_r})"
            ))
            .underline();
            ui.label(text);
            ui.label(RichText::new("    FindNewEvent(s'', s_r, p)").underline());
        } else {
            ui.label("    else Let s' be the leftmost segment of U(p) and C(p) in the StatusQueue.");
            ui.label("    Let s_l be the left neighbor of s' in the StatusQueue.");
            ui.label("    FindNewEvent(s_l, s', p)");
            ui.label("    Let s'' be the rightmost segment of U(p) and C(p) in the StatusQueue.");
            ui.label("    Let s_r be the right neighbor of s'' in the StatusQueue.");
            ui.label("    FindNewEvent(s'', s_r, p)");
        }

        ui.separator();

        let text = RichText::new("Find New Event").heading();
        ui.label(if s.typ.is_find_new_event() {
            text.underline()
        } else {
            text
        });

        if let StepType::FindNewEvent { s_l, s_r } = &s.typ {
            let s_l = segments[*s_l].id;
            let s_r = segments[*s_r].id;
            let text = RichText::new(format!("  if s_l (s{s_l}) and s_r (s{s_r}) intersect below the sweep line, or on it and to the right of the current event point p, and the intersection is not yet present as an event in the StatusQueue")).underline();
            ui.label(text);
        } else {
            ui.label("  if s_l and s_r intersect below the sweep line, or on it and to the right of the current event point p, and the intersection is not yet present as an event in the StatusQueue");
        }

        if let StepType::InsertIntersectionEvent {
            s_l,
            s_r,
            intersection: (x, y),
        } = &s.typ
        {
            let s_l = segments[*s_l].id;
            let s_r = segments[*s_r].id;
            let text = RichText::new(format!("    then insert the intersection point as an event into StatusQueue. Inserting ({y:.2}, {x:.2}, () as insection from s{s_l} and s{s_r}.")).underline();
            ui.label(text);
        } else {
            ui.label("    then Insert the intersection point as an event in StatusQueue");
        }
    }
}

fn format_segment<'a, T: A>(
    a: impl Iterator<Item = &'a SegmentIdx>,
    segments: &Segments<T>,
) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    let mut s = a;

    if let Some(s) = s.next() {
        let _ = write!(&mut buf, "s{}", segments[*s].id);
    }
    for s in s {
        let _ = write!(&mut buf, ", s{}", segments[*s].id);
    }

    buf
}
