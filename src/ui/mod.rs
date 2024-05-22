use crate::game::game::RoundCount;
use crate::game::scores::DefaultScores;
use std::{fs::File, io::Write, path::Path};

use charming::{
    component::{AngleAxis, Legend, PolarCoordinate, RadiusAxis, Title},
    element::{AxisPointer, AxisPointerType, AxisType, CoordinateSystem, Tooltip, Trigger},
    series::Line,
    Chart, HtmlRenderer,
};

pub struct ModelComparision {
    pub player_one_name: String,
    pub player_two_name: String,
    pub round_history: crate::game::game::RoundOutcomes,
}

impl ModelComparision {
    fn draw_chart(&self) -> charming::Chart {
        let data = (0..=360)
            .into_iter()
            .map(|i| {
                let t = i as f64 / 180.0 * std::f64::consts::PI;
                let r = (2.0 * t).sin() * (2.0 * t).cos();
                vec![r, i as f64]
            })
            .collect::<Vec<_>>();

        Chart::new()
            .title(Title::new().text("Two Value-Axes in Polar"))
            .legend(Legend::new().data(vec!["line"]))
            .polar(PolarCoordinate::new().center(vec!["50%", "54%"]))
            .tooltip(
                Tooltip::new()
                    .trigger(Trigger::Axis)
                    .axis_pointer(AxisPointer::new().type_(AxisPointerType::Cross)),
            )
            .angle_axis(AngleAxis::new().type_(AxisType::Value).start_angle(0))
            .radius_axis(RadiusAxis::new().min(0))
            .series(
                Line::new()
                    .name("line")
                    .coordinate_system(CoordinateSystem::Polar)
                    .show_symbol(false)
                    .data(data),
            )
    }

    pub fn draw_html(&self, file_path: &Path) -> Result<(), charming::EchartsError> {
        let chart = self.draw_chart();

        let title = format!(
            "Prisioners one dillema for {} vs {} in {} rounds",
            self.player_one_name,
            self.player_two_name,
            self.round_history.len()
        );
        let mut renderer = HtmlRenderer::new(title, 900, 900);
        renderer.save(&chart, file_path)?;

        Ok(())
    }
}
