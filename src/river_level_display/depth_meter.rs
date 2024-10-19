use yew::prelude::*;

#[function_component(RiverLevelMarkers)]
pub fn river_level_markers() -> Html {
    html! {
        <div class="depth_meter_container">
            <DepthMeter position={MeterPosition::Left}/>
            <DepthMeter position={MeterPosition::Right}/>
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
    pub position: MeterPosition
}

#[function_component(DepthMeter)]
fn depth_meter(DepthMeterProps {position}: &DepthMeterProps) -> Html {
    let position_class = match position {
        MeterPosition::Left => "left",
        MeterPosition::Right => "right"
    };

    let marker_class = format!("marker {}", position_class);

    let small_markings = (1..=4).map(|_| html! {<SmallMarking/>}).collect::<Html>();

    let all_markings = (1..=5).rev().map(
        |height| html! {
            <>
                <LargeMarking position={*position} height={height}/>
                {small_markings.clone()}
            </>
        }
    ).collect::<Html>();

    html! {
        <div class={marker_class}>
            {all_markings}
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
