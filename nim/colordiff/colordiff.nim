import os, strutils, terminal

proc readLines(path: string): seq[string] =
  result = @[]
  for line in lines(path):
    result.add(line)

proc diffFiles(fileA, fileB: string) =
  let
    linesA = readLines(fileA)
    linesB = readLines(fileB)

  var
    i = 0
    j = 0

  while i < linesA.len or j < linesB.len:
    if i < linesA.len and j < linesB.len:
      if linesA[i] == linesB[j]:
        echo linesA[i]  # identical, no color
        i.inc
        j.inc
      else:
        setForegroundColor(fgRed)
        echo "- ", linesA[i]
        resetAttributes()
        setForegroundColor(fgGreen)
        echo "+ ", linesB[j]
        resetAttributes()
        i.inc
        j.inc
    elif i < linesA.len:
      setForegroundColor(fgRed)
      echo "- ", linesA[i]
      resetAttributes()
      i.inc
    elif j < linesB.len:
      setForegroundColor(fgGreen)
      echo "+ ", linesB[j]
      resetAttributes()
      j.inc

when isMainModule:
  if paramCount() != 2:
    echo "Usage: colordiff <file1> <file2>"
    quit(1)
  diffFiles(paramStr(1), paramStr(2))