# 统计当前用户今年的提交
git-report --username "yourname" --range year

# 统计本月提交，JSON格式输出
git-report --username "yourname" --range month --format json

# 在指定目录搜索仓库
git-report --username "yourname" --root ~/projects