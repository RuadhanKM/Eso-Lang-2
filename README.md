# Eso-Lang-2
My second esoteric programming language, although calling this version esoteric is a bit generous. This one is designed to be a little bit more user friendly. 

**Currently, it's only a tokenizer.**

Here is a table of the planned operators. Some of them are weird because I needed them to be 1 character each. Create a new issue if you have a better idea for one of the operators.

|Operation|Character|
|---|---|
|`Assignment`|`=`|
|`Dot`|`.`|
|`Equality`|`~`|
|`And`|`&`|
|`Or`|`\|`|
|`Not`|`!`|
|`Less`|`<`|
|`Greater`|`>`|
|`Add`|`+`|
|`Subtract`|`-`|
|`Multiply`|`*`|
|`Divide`|`/`|

This list is _exhaustive_, so if you try `+=` or `%` you'll get an error.

However there are still 5 more tokens.

|Description|Token|
|---|---|
|`Parentheses`|`(...)`|
|`Block`|`{...}`|
|`String`|`"..."`|
|`Number`|`123`|
|`Variable`|`foo_bar`|

# *Planned* Syntax
This is how I want the syntax to look, but as I continue writing I'll probably change some stuff and forget to update the README, so keep that in mind. Not all features present in the following syntax have been implemented in the tokenizer.

- Create some variables
```
foo = "bar";
baz = 12345;
```
- Create a function with no parameters
```
foo = {
    print("Hi");
};
```
- Function with parameters
```
foo = (bar baz){
    print(bar + baz);
};
```
- Call a function
```
foo("Hello" "World");
```