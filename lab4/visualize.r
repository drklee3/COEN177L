library(ggplot2)
library(reshape2)
theme_set(theme_bw())
getwd()

data <- read.csv("test.csv")

chart_data <- melt(data, id="Size")
names(chart_data) <- c("x", "Algorithm", "value")

# plot 
ggplot() +
  geom_line(data = chart_data,
    aes(x = x, y = value, color = Algorithm),
    size = 1) +
  labs(title="Hit Rate of Page Replacement Algorithms", 
    y = "Hit Rate %", 
    x = "Table size",
    color = NULL)

ggsave("plot.png")
