library(ggplot2)
library(reshape2)
theme_set(theme_minimal())
getwd()

multiMerge = function() {
  filenames = list.files(
    path = "./data/",
    pattern="output.*.csv",
    full.names = TRUE
  )

  datalist = lapply(filenames, function(x) {
    read.csv(
      file = x,
      header = TRUE,
      stringsAsFactors = FALSE
    )
  })

  Reduce(function(x,y) {
    merge(x, y, all = TRUE, id="table_size")
  }, datalist)
}

# horizontal merge csv files
# ex: size,fifo,lru,sc
data <- multiMerge()
head(data)
write.csv(data, "./data/algorithm_data.csv", row.names=FALSE)

# convert to size, algorithm, value
chart_data <- melt(data, id="table_size")
names(chart_data) <- c("size", "Algorithm", "value")
head(chart_data)

# plot 
ggplot() +
  geom_line(
    data = chart_data,
    aes(
      x = size,
      y = value,
      color = Algorithm
    ),
    size = .5,
    alpha=0.80
  ) +
  labs(
    title="Hit Rate of Page Replacement Algorithms", 
    y = "Hit Rate",
    x = "Table / Memory Size",
    color = NULL
  )

ggsave("plot.png", device = png(), width = 8, height = 6)
dev.off()
