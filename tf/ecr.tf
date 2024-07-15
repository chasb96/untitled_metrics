resource "aws_ecrpublic_repository" "ecr_metrics" {
  provider = aws.us_east_1

  repository_name = "6ec0ee89e06a67f2096229664d07b021_metrics"
}