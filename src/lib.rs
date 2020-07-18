use rdxl::{xtype,xrender};

xtype!(
   <!Feed xml_version:String={{"1.0".to_string()}} xml_encoding:String={{"utf-8".to_string()}} xmlns:String={{"http://www.w3.org/2005/Atom".to_string()}}>
     <!FeedTitle title:String/>
     <!FeedSubtitle subtitle:String/>
     <!FeedLink href:String rel:String={{"".to_string()}}/>
     <!FeedId id:String/>
     <!FeedUpdated date:String/>
     <!FeedEntry>
       <!FeedEntryTitle title:String/>
       <!FeedEntryLink href:String rel:String={{"".to_string()}} />
       <!FeedEntryId id:String/>
       <!FeedEntryUpdated date:String/>
       <!FeedEntrySummary summary:String/>
       <!FeedEntryContentXhtml>
         <?/>
       </FeedEntryContentXhtml>
       <!FeedEntryAuthor>
         <!FeedEntryAuthorName name:String/>
         <!FeedEntryAuthorEmail email:String/>
       </FeedEntryAuthor>
     </FeedEntry>
   </Feed>
);

xrender!(Feed,
   {{format!(r#"<?xml version="{}" encoding="{}"?>"#, self.xml_version, self.xml_encoding)}}
   <feed xmlns={{self.xmlns}}>
     {{ for c in self.children.iter() {{
       {{ if let FeedChildren::FeedTitle(t) = c {{
         <title>{{ t.title }}</title>
       }} else if let FeedChildren::FeedSubtitle(t) = c {{
         <subtitle>{{ t.subtitle }}</subtitle>
       }} else if let FeedChildren::FeedLink(t) = c {{
         <link href={{t.href}} {{ if t.rel.len()>0 {{ rel={{t.rel}} }}}} />
       }} else if let FeedChildren::FeedId(t) = c {{
         <id>{{ t.id }}</id>
       }} else if let FeedChildren::FeedUpdated(t) = c {{
         <updated>{{ t.date }}</updated>
       }} else if let FeedChildren::FeedEntry(t) = c {{
         <entry>
           {{ for ec in t.children.iter() {{
             {{ if let FeedEntryChildren::FeedEntryTitle(e) = ec {{
               <title>{{ e.title }}</title>
             }} else if let FeedEntryChildren::FeedEntryLink(e) = ec {{
               <link href={{e.href}} {{ if e.rel.len()>0 {{ rel={{e.rel}} }}}} />
             }} else if let FeedEntryChildren::FeedEntryId(e) = ec {{
               <id>{{ e.id }}</id>
             }} else if let FeedEntryChildren::FeedEntryUpdated(e) = ec {{
               <updated>{{ e.date }}</updated>
             }} else if let FeedEntryChildren::FeedEntrySummary(e) = ec {{
               <summary>{{ e.summary }}</summary>
             }} else if let FeedEntryChildren::FeedEntryContentXhtml(e) = ec {{
               <content type="xhtml">
                 {{ for c in e.children.iter() {{
                   {{ let FeedEntryContentXhtmlChildren::Display(d) = c; }}
                   {{ d }}
                 }} }}
               </content>
             }} else if let FeedEntryChildren::FeedEntryAuthor(e) = ec {{
               <author>
                 {{ for c in e.children.iter() {{
                   {{ if let FeedEntryAuthorChildren::FeedEntryAuthorName(n) = c {{
                     <name>{{ n.name }}</name>
                   }} else if let FeedEntryAuthorChildren::FeedEntryAuthorEmail(n) = c {{
                     <email>{{ n.email }}</email>
                   }} }}
                 }} }}
               </author>
             }} }}
           }} }}
         </entry>
       }} }}
     }} }}
   </feed>
);
