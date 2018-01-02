# Project Builder

## how to install
1. download Project Builder from [a download link]()
2. add bin into you system path
3. use pd -v to test project builder it should print the version of project builder like (version 0.0.0)

## how to add project template
### add in folder
1. goto pb dir (you could see it by pb -l)
2. add you template project in it
## how to use pb
1. pb you_template_name (it will ask you some variable and build the template in current dir)

## how to write a template
now you could use Django syntax in template (folder name ,file name, file content etc) detail in [doc](https://tera.netlify.com/docs/installation/)

## how to use another's template
use pd add-template-repo ${git-repo}