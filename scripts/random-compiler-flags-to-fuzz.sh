#!/bin/bash

for i in $(seq 1 $1); do

case $(( RANDOM % 3 )) in
0) printf -- "-march=rv64gcv" ;;
1) printf -- "-march=rv64gcv_zvl256b" ;;
2) printf -- "-march=rv64gcv_zvl512b" ;;
2) printf -- "-march=rv64gcv_zvl1024b" ;;
esac
printf -- " "

case $(( RANDOM % 2 )) in
0) printf -- "" ;;
1) printf -- "-mtune=generic-ooo" ;;
esac
printf -- " "

case $(( RANDOM % 2 )) in
0) printf -- "" ;;
1) printf -- "-flto" ;;
esac
printf -- " "

case $(( RANDOM % 2 )) in
0) printf -- "" ;;
1) printf -- "-mrvv-vector-bits=zvl" ;;
esac
printf -- " "

if [ -z "$2" ]
then
  case $(( RANDOM % 5 )) in
  0) printf -- "" ;;
  1) printf -- "-mrvv-max-lmul=dynamic" ;;
  2) printf -- "-mrvv-max-lmul=m1" ;;
  3) printf -- "-mrvv-max-lmul=m2" ;;
  4) printf -- "-mrvv-max-lmul=m8" ;;
  esac
  printf -- " "
fi

printf -- "-O3 "

printf "\n"

done
