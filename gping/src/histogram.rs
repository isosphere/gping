use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Widget},
};

#[derive(Debug)]
pub struct HistogramState {
    /// the raw data used to compute the histogram
    /// the length of this cannot exceed if let Some(window_size) = self.window_size
    pub samples : Vec<u64>,
    /// how many samples to use when generating the historgram
    /// if None, all samples will be used without limit.
    pub window_size : Option<usize>,
    pub bin_buckets : Vec<u64>,
    pub bin_counts : Vec<u64>,
    plot_data: Vec<(f64, f64)>,
}

impl Default for HistogramState {
    fn default() -> Self {
        let bin_buckets = vec![0, 10, 100, 250, 500, 750, 1000, 1250, 1500, 1750, 2000, 5000];

        HistogramState { 
            samples: Vec::new(), 
            window_size: Some(120), 
            bin_counts: vec![0; bin_buckets.len()],
            plot_data: Vec::new(),
            bin_buckets,
        }
    }
}

impl HistogramState {
    fn _bin_index(&self, x: &u64) -> usize {
        for i in 0 .. self.bin_buckets.len() {
            if *x < self.bin_buckets[i] {
                return i
            }
        }

        self.bin_buckets.len() - 1
    }

    pub fn add_sample(&mut self, x: &u64) {
        self.samples.push(*x);
        
        // roll window
        if let Some(window) = self.window_size {
            while self.samples.len() > window {
                self.samples.remove(0);
            }
        }

        self.update()
    }

    //  FIXME: not efficient, recalculates from scratch 
    fn update_bins(&mut self) {
        // initialize
        let n = self.bin_counts.len();

        self.bin_counts = Vec::with_capacity(n);
        self.bin_counts.resize(n, 0);
        
        // count
        for i in self.samples.iter() {
            let idx = self._bin_index(i);
            self.bin_counts[idx] += 1
        }
    }

    fn update(&mut self) {
        self.update_bins();

        self.plot_data = self.bin_buckets.iter().map(|x| *x as f64).zip(self.bin_counts.iter().map(|x| *x as f64)).collect();
    }

    fn dataset(&self) -> Dataset<'_> {
        Dataset::default()
            .marker(symbols::Marker::HalfBlock)
            .style(Style::new().fg(Color::LightMagenta))
            .graph_type(GraphType::Bar)
            .data(&self.plot_data) 
    }

    pub fn render_histogram(&self, area: &Rect, buffer: &mut Buffer) {
        let max_bin = self.bin_buckets.iter().last().map_or(0, |x| *x);
        let max_count = self.window_size.iter().map(|x| *x as u64).max().unwrap_or(0);

        let dataset = self.dataset();

        Chart::new(vec![dataset])
            .block(Block::bordered().title_top(Line::from("Response Histogram (ms)").bold().centered()))
            .x_axis(
                Axis::default()
                .bounds([0.0, max_bin as f64])
            )
            .y_axis(
                Axis::default()
                    .title("Count")
                    .bounds([0.0, max_count as f64])
            )
            .render(*area, buffer);
    }
}