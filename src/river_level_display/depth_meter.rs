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

    let height_style = format!(
        "top: {}%; bottom: {}%", 
        100.0 - height_to_screenspace::get_flood_percentage(meter_max_height as f64, *meter_range) * 100.0,
        height_to_screenspace::get_flood_percentage(-1.0, *meter_range) * 100.0
    );

    let marker_column_class = format!(
        "marker_column {}", 
        match position {
            MeterPosition::Left => "left",
            MeterPosition::Right => "right"
        }
    );

    html! {
        <div class={marker_column_class}>
            <div class="marker" style={height_style}>
                {
                    (0..=meter_max_height)
                    .rev()
                    .map(|height| 
                        html! {
                            <DepthMeterSegment
                                position={*position}
                                height_label={height.to_string()}
                            />
                        }
                    ).collect::<Html>()}
                <LargeMarking position={*position} height_label={"-1"}/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DepthMeterSegmentProps {
    pub position: MeterPosition,
    pub height_label: AttrValue
}

#[function_component(DepthMeterSegment)]
fn depth_meter_segment(DepthMeterSegmentProps{position, height_label}: &DepthMeterSegmentProps) -> Html {
    html! {
        <>
            <LargeMarking position={*position} height_label={height_label}/>
            {(1..=4).map(|_| html! {<SmallMarking/>}).collect::<Html>()}
        </>
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
    pub height_label: AttrValue
}

#[function_component(LargeMarking)]
fn large_marking(LargeMarkingProps {position, height_label}: &LargeMarkingProps) -> Html {
    let height_label = html! { <div class="height_label">{format!("{}m", height_label)}</div> };

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
