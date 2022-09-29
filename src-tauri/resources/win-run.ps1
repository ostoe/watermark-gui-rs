# return "Hello, $args" # "Hello, -b X:\\Z7\\001\\2022_07_19_016_DSC_0610.JPG\r"
$result = &".\resources\exiftool.exe" $args
# write-host;
return  $result