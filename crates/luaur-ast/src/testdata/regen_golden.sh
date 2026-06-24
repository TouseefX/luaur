#!/usr/bin/env bash
# Regenerate the lexer oracle golden files from the real Luau C++ lexer.
# Run from the repo root. Requires clang++ and the vendored `luau/` sources.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
HERE="$ROOT/translation/skeleton/crates/luau-ast/src/testdata"
SRC=/tmp/luau_lex_oracle.cpp
cat > "$SRC" <<'CPP'
#include "Luau/Lexer.h"
#include "Luau/Allocator.h"
#include <cstdio>
#include <string>
using namespace Luau;
static bool isDataType(int t){return t==Lexeme::RawString||t==Lexeme::QuotedString||t==Lexeme::InterpStringBegin||t==Lexeme::InterpStringMid||t==Lexeme::InterpStringEnd||t==Lexeme::InterpStringSimple||t==Lexeme::BrokenInterpDoubleBrace||t==Lexeme::Number||t==Lexeme::Comment||t==Lexeme::BlockComment;}
static bool isNameType(int t){return t==Lexeme::Name||t==Lexeme::Attribute||(t>=Lexeme::Reserved_BEGIN&&t<Lexeme::Reserved_END);}
static void printHex(const char*p,size_t n){for(size_t i=0;i<n;++i)printf("%02x",(unsigned char)p[i]);}
int main(int argc,char**argv){if(argc<2)return 2;FILE*f=fopen(argv[1],"rb");if(!f)return 2;std::string s;char b[4096];size_t r;while((r=fread(b,1,sizeof(b),f))>0)s.append(b,r);fclose(f);
Allocator a;AstNameTable n(a);Lexer lx(s.data(),s.size(),n);
while(true){const Lexeme&l=lx.next();int t=(int)l.type;printf("%d|%u|%u|%u|%u|",t,l.location.begin.line,l.location.begin.column,l.location.end.line,l.location.end.column);
if(isDataType(t))printHex(l.data,l.getLength());else if(isNameType(t)&&l.name)printHex(l.name,strlen(l.name));printf("\n");if(t==Lexeme::Eof)break;}return 0;}
namespace Luau{int gAstRttiIndex=0;}
CPP
clang++ -std=c++17 -O0 -I "$ROOT/luau/Ast/include" -I "$ROOT/luau/Common/include" \
  "$SRC" "$ROOT/luau/Ast/src/Lexer.cpp" "$ROOT/luau/Ast/src/Allocator.cpp" \
  "$ROOT/luau/Ast/src/Confusables.cpp" "$ROOT/luau/Ast/src/Location.cpp" \
  "$ROOT/luau/Common/src/StringUtils.cpp" -o /tmp/luau_lex_oracle
for fx in "$HERE"/*.luau; do /tmp/luau_lex_oracle "$fx" > "${fx%.luau}.golden"; done
echo "regenerated goldens for: $(ls "$HERE"/*.luau)"
