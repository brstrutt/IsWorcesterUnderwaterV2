use yew::prelude::*;

use crate::river_level_display::height_to_screenspace;

#[derive(Properties, PartialEq)]
pub struct RiverLevelMarkersProps {
    pub left_marker_range: height_to_screenspace::HeightRange,
    pub right_marker_range: height_to_screenspace::HeightRange
}

#[function_component(RiverLevelMarkers)]
pub fn river_level_markers(RiverLevelMarkersProps {left_marker_range, right_marker_range}: &RiverLevelMarkersProps) -> Html {
    html! {
        <div class="depth_meter_container">
            <DepthMeter position={MeterPosition::Left} meter_range={*left_marker_range}/>
            <DepthMeter position={MeterPosition::Right} meter_range={*right_marker_range}/>
        </div>
    }
}

#[derive(PartialEq, Clone, Copy)]
enum MeterPosition {
    Left,
    Right
}

#[derive(Properties, PartialEq)]
struct DepthMeterProps {
    pub position: MeterPosition,
    pub meter_range: height_to_screenspace::HeightRange
}

#[function_component(DepthMeter)]
fn depth_meter(DepthMeterProps {position, meter_range}: &DepthMeterProps) -> Html {
    let meter_max_height = 6;

    let base_height = height_to_screenspace::get_flood_percentage(0.0, *meter_range);
    let top_height = height_to_screenspace::get_flood_percentage(meter_max_height as f64, *meter_range);

    let height_style = format!("top: {}%; bottom: {}%", 100.0 - top_height*100.0, base_height*100.0);

    let position_class = match position {
        MeterPosition::Left => "left",
        MeterPosition::Right => "right"
    };

    let marker_column_class = format!("marker_column {}", position_class);

    let small_markings = (1..=4).map(|_| html! {<SmallMarking/>}).collect::<Html>();

    let all_markings = (1..=meter_max_height).rev().map(
        |height| html! {
            <>
                <LargeMarking position={*position} height={height}/>
                {small_markings.clone()}
            </>
        }
    ).collect::<Html>();

    html! {
        <div class={marker_column_class}>
            <div class="marker" style={height_style}>
                {all_markings}
                <LargeMarking position={*position} height={0}/>
            </div>
        </div>
    }
}

#[function_component(SmallMarking)]
fn small_marking() -> Html {
    html! {
        <div class="depth_row">
            <div class="block small"/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LargeMarkingProps {
    pub position: MeterPosition,
    pub height: u32
}

#[function_component(LargeMarking)]
fn large_marking(LargeMarkingProps {position, height}: &LargeMarkingProps) -> Html {
    let height_label = html! { <div class="height_label">{format!("{}m", height)}</div> };

    match position {
        MeterPosition::Left => html! {
            <div class="depth_row">
                <div class="block large"/>
                {height_label}
            </div>
        },
        MeterPosition::Right => html! {
            <div class="depth_row">
                {height_label}
                <div class="block large"/>
            </div>
        },
    }
}
