
# /^[a-z]*$/ {

    split($0, letters, "");
    key = ""
    asort(letters, sorted)
    for (i in sorted) {
      key = key sorted[i]
    }
    if (words[key])
        words[key] = words[key] "," $0
    else
        words[key] = $0
}

END {
    for (key in words) {
        if (words[key] ~ /,/)
            print words[key]
    }
}




