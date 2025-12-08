# frozen_string_literal: true

lines = File.readlines('input.txt')
part_one = true
ranges = []
ids = []
lines.each do |line|
  if line.strip.empty?
    part_one = false
    next
  end

  if part_one
    ranges << (line.strip.split('-').map(&:to_i))
  else
    ids << line.strip.to_i
  end
end

# Sort ranges by start value
ranges.sort_by! { |range| range[0] }

# puts "Ranges: #{ranges.inspect}"
# puts "IDs: #{ids.inspect}"

# Merge overlapping ranges
merged_ranges = []
current_range = ranges.first

ranges[1..].each do |range|
  if range[0] <= current_range[1] + 1
    # Overlapping or adjacent, extend current range
    current_range[1] = [current_range[1], range[1]].max
  else
    # No overlap, save current and start new range
    merged_ranges << current_range
    current_range = range
  end
end

merged_ranges << current_range

# puts "Merged Ranges: #{merged_ranges.inspect}"

# Count IDs not in merged ranges
fresh = ids.select do |id|
  merged_ranges.any? { |range| id >= range[0] && id <= range[1] }
end

# puts "IDs in merged ranges: #{fresh.inspect}"
puts "Count of IDs in merged ranges: #{fresh.size}"

# Count total numbers in merged ranges
total_size_of_ranges = merged_ranges.reduce(0) do |sum, range|
  sum + (range[1] - range[0] + 1)
end

puts "Total size of merged ranges: #{total_size_of_ranges}"