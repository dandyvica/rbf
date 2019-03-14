use crate::record::Record;
use crate::layout::Layout;

pub trait Exportable {
    fn to_html(&self) -> String;
}

impl<T> Exportable for Record<T> {
    /// Converts a record to an HTML table with all fields data.
    fn to_html(&self) -> String {
        let mut s = String::with_capacity(200 * self.count());

        // record description
        s += format!(
            "<h2><span class=\"label label-primary\">{}-{}-{}</span></h2>",
            self.name, self.description, self.calculated_length
        )
        .as_str();

        // fields description
        s += format!("<table class=\"table table-striped\">").as_str();
        s += format!("<thead><tr><th>#</th><th>Field name</th><th>Description</th>").as_str();
        s +=
            format!("<th>Type</th><th>Length</th><th>Start</th><th>End</th></tr></thead>").as_str();

        for f in self {
            s += format!(
                "<tr><td>{}</td><td><strong>{}</strong></td>",
                f.index + 1,
                &f.name
            )
            .as_str();
            s += format!(
                "<td>{}</td><td>{}</td><td>{}</td>",
                &f.description, f.ftype.id, f.length
            )
            .as_str();
            s += format!(
                "<td>{}</td><td>{}</td></tr>",
                f.lower_offset + 1,
                f.upper_offset + 1
            )
            .as_str();
        }

        // close HTML table
        s += format!("</table>").as_str();

        s
    }
}

impl<T> Exportable for Layout<T> {
    /// Converts the whole layout to an HTML page. Records are sorted alphanumerically, and HTML is
    /// using somme bootstrap goodies. Write HTML into an output file.
    #[allow(unused_must_use)]
    fn to_html(&self) -> String {
        let mut s = String::with_capacity(2000);

        // write out HTML header (uses bootstrap css framework)
        s += format!(
            r#"<!DOCTYPE output><html lang="en"><head><meta charset="utf-8">"#
        ).as_str();
        s += format!(r#"<link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.5/css/bootstrap.min.css"></head>"#).as_str();
        s += format!(
            r#"<style>{}</style>"#,
            "@media print { h2 {page-break-before: always;} }"
        ).as_str();
        s += format!(r#"<body role="document"><div class="container theme-showcase" role="main">"#).as_str();
        s += format!(
            r#"<div class="jumbotron"><h1 class="text-center">{} ({})</h1></div>"#,
            self.description, self.version
        ).as_str();
        s += format!(r#"<div class="container">"#).as_str();

        // now write out field types
        let mut ftypes: Vec<_> = self.ftypes.keys().collect();
        ftypes.sort();

        // meta data
        s += format!(r#"<h2><span class="label label-primary">Metadata</span></h2>"#).as_str();
        s += format!(r#"<table class="table table-striped">"#).as_str();

        s += format!(
            r#"<tr><td><strong>{}</strong></td><td>{}</td></tr>"#,
            "Record length", self.rec_length
        ).as_str();
        s += format!(
            r#"<tr><td><strong>{}</strong></td><td>{}</td></tr>"#,
            "Version", self.version
        ).as_str();
        s += format!(
            r#"<tr><td><strong>{}</strong></td><td>{}</td></tr>"#,
            "Description", self.description
        ).as_str();
        s += format!(
            r#"<tr><td><strong>{}</strong></td><td>{}</td></tr>"#,
            "schema", self.schema
        ).as_str();

        s += format!(r#"</table><br>"#).as_str();

        // fields description
        s += format!(r#"<h2><span class="label label-primary">Field types</span></h2>"#).as_str();
        s += format!(r#"<table class="table table-striped">"#).as_str();
        s += format!(
            r#"<thead><tr><th>Field type</th><th>Type of data</th><th>Pattern</th></tr></thead>"#
        ).as_str();

        for ftype in ftypes {
            s += format!(
                r#"<tr><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                self.ftypes.get(ftype).unwrap().id,
                self.ftypes.get(ftype).unwrap().type_as_string,
                self.ftypes.get(ftype).unwrap().pattern.as_str()
            ).as_str();
        }

        s += format!(r#"</table><br>"#).as_str();

        // now write records, sorted.
        let mut rec_names: Vec<_> = self.rec_map.keys().collect();
        rec_names.sort();

        for recname in rec_names {
            s += format!("{}", self.get(&recname).unwrap().to_html()).as_str();
            s += format!("<br>").as_str();
        }

        s += format!("{}", "</div></div></body></html>").as_str();

        s
    }
}
