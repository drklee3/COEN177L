library(ggplot2)
library(reshape2)
theme_set(theme_minimal())
getwd()

multiMerge = function() {
  filenames = list.files(path = "./", pattern="*.csv", full.names = TRUE)
  datalist = lapply(filenames, function(x) {
    read.csv(file = x, header = TRUE, stringsAsFactors = FALSE)
  })
  Reduce(function(x,y) {
    merge(x, y, all = TRUE, id="table_size")
  }, datalist)
}

data <- multiMerge()
head(data)

chart_data <- melt(data, id="table_size")
head(chart_data)
names(chart_data) <- c("size", "Algorithm", "value")

# plot 
ggplot() +
  geom_line(data = chart_data,
    aes(x = size, y = value, color = Algorithm),
    size = .5,
    alpha=0.80) +
  labs(title="Hit Rate of Page Replacement Algorithms", 
    y = "Hit Rate", 
    x = "Table size",
    color = NULL)

ggsave("plot.png", device = png(), width = 8, height = 6)
dev.off()
