require "yaml"

PRELUDE = <<BEGIN
// Generated file. Do not edit.
// File generated #{Time.now}
BEGIN

# Load up YAML file
if ARGV.length < 2 then
  abort "Usage: [yamldesc] [targetfile]"
end

data = Psych.load_file(ARGV[0])
puts data

def check_if_present(data, name)
  abort "Missing data: #{name}, from #{data}" unless data.has_key?(name)
end

def check_if_present?(data, name)
  data.has_key?(name)
end

str = ""

check_if_present(data, "derives")
check_if_present(data, "struct_name")
check_if_present(data, "variants")
data["variants"].each do |x|
  check_if_present(x, "name")
  check_if_present(x, "members")
  x["members"].each do |y|
    check_if_present(y, "name")
    check_if_present(y, "type")
  end unless x["members"].nil?
end
Members = Struct.new(:name, :members) do
end
derives = data["derives"]
struct_name = data["struct_name"]
members = data["variants"].map { |e| Members::new(e["name"])  }

puts members

# Assume all the needed things are in the data file and get going!
str << data["use"] << "\n" if check_if_present?(data, "use")
str << "\#[derive(#{derives.join(", ")})]\n"
str << "enum #{struct_name} {\n"
members.each do |m|
  if m["members"].nil?
    str << "\t#{m.name},\n"
  else
    str << "\t#{m.name}(#{m.name}),\n"
  end
end
str << "}\n"
fl = File.open(ARGV[1], "w")
fl << PRELUDE << "\n"
fl << str

puts PRELUDE
puts str
