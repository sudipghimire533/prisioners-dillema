use crate::game::game::RoundCount;
use crate::game::scores::DefaultScores;
use std::{fs::File, io::Write, path::Path};

use charming::{
    component::{AngleAxis, Axis, Legend, PolarCoordinate, RadiusAxis, Title},
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
        let title = format!(
            "Point accumilation for {} vs {} in {} rounds",
            self.player_one_name,
            self.player_two_name,
            self.round_history.len()
        );

        let player_one_data = self
            .round_history
            .iter()
            .map(|a| a.player_one_point as i64)
            .collect();
        let player_two_data = self
            .round_history
            .iter()
            .map(|a| a.player_two_point as i64)
            .collect();

        Chart::new()
            .title(Title::new().text(title))
            .x_axis(Axis::new().type_(AxisType::Category))
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().smooth(0.5).data(player_one_data))
            .series(Line::new().smooth(0.5).data(player_two_data))
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
