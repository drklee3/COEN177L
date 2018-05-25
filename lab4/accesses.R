library(ggplot2)
library(reshape2)
theme_set(theme_minimal())
getwd()

data <- read.csv(file = "accesses.txt")
data <- as.data.frame(table(data),decreasing=T)
names(data) <- c("pageRequest", "frequency")
head(data)

# plot 
ggplot() +
  geom_point(
    data = data,
    aes(
      x = pageRequest,
      y = frequency,
      size = frequency
    )
  ) +
  # size dots 0 - 2
  scale_size_continuous(range = c(0, 3)) +
  # set x labels to every 1k
  scale_x_discrete(breaks = seq(0, 10000, by = 1000)) +
  labs(
    title="Page Requests", 
    y = "Number of Requests",
    x = "Page Request",
    color = NULL
  )

ggsave("accesses.png", device = png(), width = 8, height = 6)
dev.off()
