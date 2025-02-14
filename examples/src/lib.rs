use chart_js_rs::{
    bar::Bar, doughnut::Doughnut, line::Line, pie::Pie, scatter::Scatter, ChartOptions, Dataset,
    NoAnnotations, SinglePointDataset, XYDataset, XYPoint,
};
use dominator::{self, events, html, Dom};
use futures_signals::signal::{Mutable, MutableSignalCloned, Signal, SignalExt};
use rand::Rng;
use std::{
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

fn random() -> Vec<(usize, usize)> {
    let rng = rand::thread_rng();

    let rnd_y = (0..20).map(|_| rng.clone().gen_range(0..100));
    let rnd_x = (0..20).map(|_| rng.clone().gen_range(0..10));
    rnd_x.zip(rnd_y).collect()
}

#[derive(Debug, Clone)]
pub struct Model {
    chart: Mutable<&'static str>,
    data: Mutable<Rc<Vec<(usize, usize)>>>,
    data_2: Mutable<Rc<Vec<(usize, usize)>>>,
}
impl Model {
    async fn init() -> Rc<Self> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        Rc::new(Model {
            chart: Mutable::new("chart_one"),
            data: Mutable::new(Rc::new(random())),
            data_2: Mutable::new(Rc::new(random())),
        })
    }

    fn chart_not_selected(self: Rc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c != chart)
    }

    fn show_charts(self: Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        Mutable3::new(self.chart.clone(), self.data.clone(), self.data_2.clone()).map(
            move |(c, data, data_2)| match c.to_string().as_str() {
                "chart_one" => Some(self.clone().show_chart_one(data.to_vec(), data_2.to_vec())),
                "chart_two" => Some(self.clone().show_chart_two(data.to_vec())),
                "chart_three" => Some(self.clone().show_chart_three()),
                "chart_line" => Some(self.clone().show_chart_line(data.to_vec(), false)),
                "chart_line_time" => Some(self.clone().show_chart_line(data.to_vec(), true)),
                _ => None,
            },
        )
    }

    fn show_chart_one(
        self: Rc<Self>,
        data: Vec<(usize, usize)>,
        data_2: Vec<(usize, usize)>,
    ) -> Dom {
        // construct and render chart here
        let id = "chart_one";

        let chart = Scatter::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                datasets: Vec::from([
                    XYDataset {
                        data: data
                            .iter()
                            .map(|d| XYPoint {
                                // iterate over our data to construct a dataset
                                x: d.0.into(), // use .into() to convert to a NumberorDateString
                                y: d.1.into(),
                            })
                            .collect::<Vec<_>>(), // collect into a Vec<XYPoint>

                        borderColor: "red".into(),
                        backgroundColor: "lightcoral".into(),
                        pointRadius: 4.into(),
                        label: "Dataset 1".into(),
                        ..Default::default() // always use `..Default::default()` to make sure this works in the future
                    },
                    XYDataset {
                        data: data_2
                            .iter()
                            .map(|d| XYPoint {
                                // iterate over our data to construct a dataset
                                x: d.0.into(), // use .into() to convert to a NumberorDateString
                                y: d.1.into(),
                            })
                            .collect::<Vec<_>>(), // collect into a Vec<XYPoint>

                        borderColor: "blue".into(),
                        backgroundColor: "lightskyblue".into(),
                        pointRadius: 4.into(),
                        label: "Dataset 2".into(),
                        ..Default::default() // always use `..Default::default()` to make sure this works in the future
                    },
                ]),
                ..Default::default()
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default() // always use `..Default::default()` to make sure this works in the future
            },
            id: id.into(),
            ..Default::default()
        };
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
            .prop("id", id)
            .style("height", "calc(100vh - 270px)")
            .after_inserted(move |_| {
                chart.to_chart().render_mutate() // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see chart_two and index.html
            })
        })
    }

    fn show_chart_two(self: Rc<Self>, data: Vec<(usize, usize)>) -> Dom {
        // construct and render chart here
        let id = "chart_two";

        let chart = Bar::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                labels: Some(
                    // use a range to give us our X axis labels
                    (0..data.len()).map(|d| (d + 1).into()).collect(),
                ),
                datasets: Vec::from([XYDataset {
                    data: data
                        .iter()
                        .enumerate()
                        .map(|(x, d)| XYPoint {
                            // iterate over our data to construct a dataset
                            x: (x + 1).into(), // use enumerate to give us our X axis point
                            y: d.1.into(),
                        })
                        .collect::<Vec<_>>(), // collect into a Vec<XYPoint>

                    backgroundColor: "palegreen".into(),
                    borderColor: "green".into(),
                    borderWidth: 2.into(),
                    label: "Dataset 1".into(),
                    yAxisID: "y".into(),
                    ..Default::default() // always use `..Default::default()` to make sure this works in the future
                }]),
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default() // always use `..Default::default()` to make sure this works in the future
            },
            id: id.into(),
            ..Default::default()
        };
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
            .prop("id", id)
            .style("height", "calc(100vh - 270px)")
            .after_inserted(move |_| {
                chart.to_chart().render() // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see chart_two and index.html
            })
        })
    }

    fn show_chart_three(self: Rc<Self>) -> Dom {
        // construct and render chart here
        let three_id = "chart_three_a";
        let four_id = "chart_three_b";

        let three_a_chart: Doughnut<NoAnnotations> = Doughnut {
            data: {
                Dataset {
                    datasets: {
                        Vec::from([SinglePointDataset {
                            data: Vec::from([300.into(), 40.into(), 56.into(), 22.into()]),
                            backgroundColor: Vec::from([
                                "dodgerblue".into(),
                                "limegreen".into(),
                                "firebrick".into(),
                                "goldenrod".into(),
                            ]),
                            ..Default::default()
                        }])
                    },
                    labels: Some(Vec::from([
                        "Blueberries".into(),
                        "Limes".into(),
                        "Apples".into(),
                        "Lemons".into(),
                    ])),
                }
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default()
            },
            id: three_id.to_string(),
            ..Default::default()
        };
        let three_b_chart: Pie<NoAnnotations> = Pie {
            data: {
                Dataset {
                    datasets: {
                        Vec::from([SinglePointDataset {
                            data: Vec::from([300.into(), 40.into(), 56.into(), 22.into()]),
                            backgroundColor: Vec::from([
                                "dodgerblue".into(),
                                "limegreen".into(),
                                "firebrick".into(),
                                "goldenrod".into(),
                            ]),
                            ..Default::default()
                        }])
                    },
                    labels: Some(Vec::from([
                        "Blueberries".into(),
                        "Limes".into(),
                        "Apples".into(),
                        "Lemons".into(),
                    ])),
                }
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default()
            },
            id: four_id.to_string(),
            ..Default::default()
        };
        html!("div", {
            .class("columns")
            .children([
                html!("div", {
                    .class(["column", "is-half"])
                    .child(
                        html!("canvas", {
                        .prop("id", three_id)
                        .style("height", "calc(100vh - 270px)")
                        .after_inserted(move |_| {
                            three_a_chart.to_chart().render()
                        })
                    }))
                }),
                html!("div", {
                    .class(["column", "is-half"])
                    .child(
                        html!("canvas", {
                        .prop("id", four_id)
                        .style("height", "calc(100vh - 270px)")
                        .after_inserted(move |_| {
                            three_b_chart.to_chart().render()
                        })
                    }))
                })
            ])
        })
    }

    fn show_chart_line(self: Rc<Self>, data: Vec<(usize, usize)>, time: bool) -> Dom {
        let id = if time {
            "chart_line_time"
        } else {
            "chart_line"
        };

        let chart = Line::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                datasets: Vec::from([SinglePointDataset {
                    data: data
                        .iter()
                        .map(|d| d.1.to_string().into())
                        .collect::<Vec<_>>(),

                    label: "Line 1".to_string(),
                    ..Default::default()
                }]),
                labels: Some(
                    (0..data.len())
                        .map(|x| {
                            (if time {
                                x.to_string() + ":12:15"
                            } else {
                                x.to_string()
                            })
                            .into()
                        })
                        .collect::<Vec<_>>(),
                ),
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default() // always use `..Default::default()` to make sure this works in the future
            },
            id: id.into(),
            ..Default::default()
        };
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
            .prop("id", id)
            .style("height", "calc(100vh - 270px)")
            .after_inserted(move |_| {
                chart.to_chart().render_mutate() // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see chart_two and index.html
            })
        })
    }

    fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .class("section")
            .child(
                html!("div", {
                    .class(["buttons", "has-addons"])
                    .child(
                        html!("button", {
                            .class(["button", "is-info"])
                            .text("Randomise")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().data.set(Rc::new(random())); // randomise the data on button click
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-primary"])
                            .class_signal("is-light", self.clone().chart_not_selected("chart_one"))
                            .text("Chart 1")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("chart_one"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-success"])
                            .class_signal("is-light", self.clone().chart_not_selected("chart_two"))
                            .text("Chart 2")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("chart_two"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-primary"])
                            .class_signal("is-light", self.clone().chart_not_selected("chart_three"))
                            .text("Chart 3")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("chart_three"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-primary"])
                            .class_signal("is-light", self.clone().chart_not_selected("chart_line"))
                            .text("Chart line")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("chart_line"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-primary"])
                            .class_signal("is-light", self.clone().chart_not_selected("chart_line_time"))
                            .text("Chart line(Time)")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("chart_line_time"); // change which chart is in view
                                }
                            })
                        })
                    )
                })
            )
            .child(
                html!("div", {
                    .class("section")
                    .child_signal(self.show_charts()) // render charts here, signal allows us to change the chart, see the `Dominator` crate for more
                })
            )
        })
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    let app = Model::init().await;

    dominator::append_dom(&dominator::body(), Model::render(app));

    Ok(())
}

pub struct Mutable3<A, B, C>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
)
where
    A: Clone,
    B: Clone,
    C: Clone;
impl<A, B, C> Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>) -> Self {
        Mutable3(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
        )
    }
}
impl<A, B, C> Signal for Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    type Item = (A, B, C);

    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let mut changed = false;

        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        let middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        let right_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
            )))
        } else if left_done && middle_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}
