library(ggplot2)
library(reshape2)
theme_set(theme_minimal())
getwd()

data <- read.csv(
  file = "output.csv",
  header = TRUE,
  stringsAsFactors = FALSE
)
head(data)

ns_to_ms <- function(x) { 
  x / 1000000
}

# plot 
ggplot() +
  geom_line(
    data = data,
    aes(
      x = lock_percentage,
      y = time,
      group = thread_count,
      color = factor(thread_count)
    ),
    size = .5,
    alpha=0.80
  ) +
  scale_y_continuous(labels = ns_to_ms) +
  labs(
    title="Lock Performance (1M work units)",
    y = "Time (ms)",
    x = "Lock Duration",
    color = "Threads"
  )

ggsave("plot.png", device = png(), width = 8, height = 6)
dev.off()
