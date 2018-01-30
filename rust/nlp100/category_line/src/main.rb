require "json"
File.open(ARGV.shift) do |f|
  f.each do |json|
    j = JSON.parse(json)
    puts j["text"].split(/\n/).map{|m| m =~ /\[\[Category:.+\]\]/ ? m : nil }.compact
  end
end
