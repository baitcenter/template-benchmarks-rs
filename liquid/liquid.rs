#![feature(test)]
extern crate liquid;
extern crate serde_yaml;
extern crate test;

use liquid::{Object, ParserBuilder, Value};

#[bench]
fn big_table(b: &mut test::Bencher) {
    let size = 500;
    let mut table = Vec::with_capacity(size);
    for _ in 0..size {
        let mut inner = Vec::with_capacity(size);
        for i in 0..size {
            inner.push(Value::Scalar((i as i32).into()));
        }
        table.push(Value::Array(inner));
    }

    let template = ParserBuilder::with_liquid()
        .build()
        .parse("<table>
{% for row in table %}
<tr>{% for col in row %}<td>{{ col|escape }}</td>{% endfor %}</tr>
{% endfor %}
</table>").unwrap();

    let mut globals = Object::new();
    globals.insert("table".to_string(), Value::Array(table));

    b.iter(|| template.render(&globals));
}

static TEAMS_TEMPLATE: &'static str = "<html>
  <head>
    <title>{{year}}</title>
  </head>
  <body>
    <h1>CSL {{year}}</h1>
    <ul>
    {% for team in teams %}
      <li class=\"{% if forloop.first %}champion{% endif %}\">
      <b>{{team.name}}</b>: {{team.score}}
      </li>
    {% endfor %}
    </ul>
  </body>
</html>";

static TEAMS_DATA: &'static str = "
year: 2015
teams:
  - name: Jiangsu
    score: 43
  - name: Beijing
    score: 27
  - name: Guangzhou
    score: 22
  - name: Shandong
    score: 12
";

#[bench]
fn teams(b: &mut test::Bencher) {
    let parser = ParserBuilder::with_liquid().extra_filters().build();
    let template = parser
        .parse(TEAMS_TEMPLATE)
        .unwrap();

    let data: liquid::Object =
        serde_yaml::from_str(TEAMS_DATA).unwrap();

    b.iter(|| template.render(&data));
}
