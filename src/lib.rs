use rdxl::{xtype,xrender};

xtype!(
   <!Feed xml_version:String={{"1.0".to_string()}} xml_encoding:String={{"utf-8".to_string()}} xmlns:String={{"http://www.w3.org/2005/Atom".to_string()}}>
     <!FeedTitle title:String/>
     <!FeedSubtitle subtitle:String/>
     <!FeedLink link:String/>
     <!FeedId id:String/>
     <!FeedUpdated date:String/>
     <!FeedEntry>
       <!FeedEntryTitle title:String/>
       <!FeedEntryLink link:String/>
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
   </feed>
);

/*
	<title>Example Feed</title>
	<subtitle>A subtitle.</subtitle>
	<link href="http://example.org/feed/" rel="self" />
	<link href="http://example.org/" />
	<id>urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6</id>
	<updated>2003-12-13T18:30:02Z</updated>
	
	
	<entry>
		<title>Atom-Powered Robots Run Amok</title>
		<link href="http://example.org/2003/12/13/atom03" />
		<link rel="alternate" type="text/html" href="http://example.org/2003/12/13/atom03.html"/>
		<link rel="edit" href="http://example.org/2003/12/13/atom03/edit"/>
		<id>urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a</id>
		<updated>2003-12-13T18:30:02Z</updated>
		<summary>Some text.</summary>
		<content type="xhtml">
			<div xmlns="http://www.w3.org/1999/xhtml">
				<p>This is the entry content.</p>
			</div>
		</content>
		<author>
			<name>John Doe</name>
			<email>johndoe@example.com</email>
		</author>
	</entry>

</feed>
*/
