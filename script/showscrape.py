#!/usr/bin/env python3
import re 
import itertools
import requests
from bs4 import BeautifulSoup

################################################################################
#
# html template 
#
################################################################################

html_template = '''
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Listing of Salt Lake City Music Concerts Shows">
    <meta name="keywords" content="Music, Concerts, Shows, Salt, Lake">
    <meta name="author" content="keith">

    <title>slc shows</title>
    <style>
      body {
          background-color:black;
          color:silver;
          font-family:"Courier New";
          font-size:large; 
      }
      a {
          /*text-decoration: none;*/
      }
      a:link {
          color:silver;
      }
      a:visited {
          color:gray;
      }
      tr td{
          max-width: 48ch;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
      }
   </style>
  </head>
  <body>
    <main>
        <p>---</p>  
        <table id=events_table></table>
    </main>
  </body>
</html>
'''




################################################################################
#
# Show data
#
################################################################################

import functools
from datetime import datetime

class Show:

    @functools.total_ordering
    class Date():
        def __init__(self, month, day):
            if month < 0 or month > 12:
                print(f"WARN: invalid month in ctor ({month})")
                self.month = 0
            else:
                self.month = month
            if day < 0 or day > 31:
                print(f"WARN: invalid day in ctor ({day})")
                self.day = 0
            else:
                self.day = day

        def __repr__(self):
            return f"{month_int_to_str(self.month)} {self.day:02d}"

        def __eq__(self, other):
            if not isinstance(other, Show.Date):
                return False
            return self.month == other.month and self.day == other.day

        def __lt__(self, other):
            if not isinstance(other, Show.Date):
                return False

            # TODO: cache this
            cur_month = datetime.now().month

            adjusted_self_month = self.month if self.month >= cur_month else self.month+12
            adjusted_other_month = other.month if other.month >= cur_month else other.month+12
            return ((adjusted_self_month <  adjusted_other_month) or 
                    (adjusted_self_month == adjusted_other_month and self.day < other.day)) 


    def __init__(self, artist, date, city, venue, url):
        self.artist = artist
        if not date:
            self.date = ""
        else:
            self.date = date
        if not city:
            self.city = ""
        else:
            self.city = city
        if not venue:
            self.venue = ""
        else:
            self.venue = venue

        if not url:
            self.ticket_url = ""
        else:
            self.ticket_url = url

        
    def __eq__(self, other):
        return self.date == other.date and self.artist == other.artist

    def __lt__(self, other):
        return self.date < other.date
    
    def __hash__(self):
        return hash((self.artist, self.date.month, self.date.day, self.city, self.venue))


################################################################################
#
# util 
#
################################################################################

def get_html(url):
    try: 
        response = requests.get(url)
        return BeautifulSoup(response.content, "html.parser")

    except requests.exceptions.RequestException as e:
        print(f"Request failed for url '{url}': {e}")
        return None

def month_str_to_int(month_str):
    month_str = month_str.lower()
    if month_str.startswith('ja'):
        return 1
    elif month_str.startswith('f'):
        return 2
    elif month_str.startswith('mar'):
        return 3
    elif month_str.startswith('ap'):
        return 4
    elif month_str.startswith('may'):
        return 5
    elif month_str.startswith('jun'):
        return 6
    elif month_str.startswith('jul'):
        return 7
    elif month_str.startswith('au'):
        return 8
    elif month_str.startswith('s'):
        return 9
    elif month_str.startswith('o'):
        return 10 
    elif month_str.startswith('n'):
        return 11 
    elif month_str.startswith('d'):
        return 12 
    else:
        return -1

def month_int_to_str(month_int):
    month_strs = [
        "???",
        "jan",
        "feb",
        "mar",
        "apr",
        "may",
        "jun",
        "jul",
        "aug",
        "sep",
        "oct",
        "nov",
        "dec"
    ]
    return month_strs[month_int]


def generate_html(shows):
    shows = sorted(list(set(shows)))
    soup = BeautifulSoup(html_template, "html.parser")
    table = soup.find("table")
    with open("index.html", "w", encoding='utf-8') as file:
        last_date = None
        for show in shows:
            row = soup.new_tag("tr")

            # date
            if last_date == show.date:
                row.append(BeautifulSoup(f"<td></td>", "html.parser"))
            else:
                last_date = show.date
                row.append(BeautifulSoup(f"<pre><td>{show.date}&nbsp;&nbsp;</td></pre>", "html.parser"))

            # artist
            if show.ticket_url:
                row.append(BeautifulSoup(f"<td><a href=\"{show.ticket_url}\">{show.artist}</a></td>", "html.parser"))
            else:
                row.append(BeautifulSoup(f"<td>{show.artist}</td>", "html.parser"))

            # city
            row.append(BeautifulSoup("<pre><td>&nbsp;&nbsp;</td></pre>", "html.parser"))
            row.append(BeautifulSoup(f"<td>{show.city.lower()}</td>", "html.parser"))

            # venue
            row.append(BeautifulSoup("<pre><td>&nbsp;&nbsp;</td></pre>", "html.parser"))
            row.append(BeautifulSoup(f"<td>{show.venue.lower()}</td>", "html.parser"))

            table.append(row)
        file.write(soup.prettify())

################################################################################
#
# 24Tix 
#
################################################################################


#'Thu, Feb 13 / 07:00PM'
date_re = re.compile("[a-zA-Z]+,\s+([a-zA-Z]+)\s+(\d+).+")

def parse_city_24tix(city_str):
    city_state = city_str.split(',')
    city_state = [x.strip() for x in city_state]
    #city = ", ".join(city_state) #INFO: if we want to include state ....
    city = city_state[0]
    city = city.lower()
    city = city.replace("salt lake city", "slc")
    return city

def parse_date_24tix(date_str):
    m = date_re.match(date_str)
    if m:
        month_str = m.groups()[0]
        day_str = m.groups()[1]
        return (month_str_to_int(month_str), int(day_str)) 
    else:
        print(f"WARN: Failed to regex match 24tix date str '{date_str}'")
        return (0,0) 


def process_24tix():
    print("processing 24tix ...")
    url_template_24tix = 'https://www.24tix.com/?batch_page={}'

    shows = []
    pages_processed = 0
    for i in itertools.count(start=1):
        url = url_template_24tix.format(i)
        #print(f"Trying url '{url}'")
        soup = get_html(url)
        
        html_events = soup.find_all("div", class_="card-body event-body")
        
        if html_events:
            pages_processed += 1
            for html_event in html_events:
                link = html_event.find("a")
                if not link:
                    print(f"WARN: Failed to find 24tix artist link")
                    continue
                artist = link.getText().strip()
                ticket_url = link.get('href')

                date_div =  html_event.find("div", class_="event-start")
                if not date_div:
                    print(f"WARN: Failed to find 24tix date div")
                    date = (0,0) 
                else:
                    date = parse_date_24tix(date_div.getText().strip())

                venue_block = html_event.find("div", class_="event-venue mt-3")
                if not venue_block:
                    print(f"WARN: Failed to find 24tix venue block")
                else:
                    venue_header = venue_block.find("h6")
                    if not venue_header:
                        print(f"WARN: Failed to find 24tix venue block header")
                    else:
                        venue = venue_header.getText().strip()
                city = parse_city_24tix(venue_block.find("small").getText().strip())
                shows.append(
                    Show(
                        artist, 
                        Show.Date(date[0], date[1]),
                        city,
                        venue,
                        ticket_url
                    )
                )
                #print(f"'{artist}' '{date}' '{venue}' '{city}'")

        else:
            print(f"\tBatch {i} failed") 
            break

    print(f"\tpages processed: {pages_processed}")
    print(f"\tshows found: {len(shows)}")
    return shows 

################################################################################
#
# state room presents 
#
################################################################################
'''
<div class="allevents-event my-3 shadow col col-sm-4">
    <div class="allevents-img">
        <a href="/state-room-presents/pigeons-playing-ping-pong-3">
            <span class="acfup-item">
                <img src="https://thestateroompresents.com/images/acfupload/Pigeons-Playing-Ping-Pong_03-11-2025_v2_Facebook_1200x628.jpg"/>
            </span>
        </a>
    </div>
    <div class="p-3">
        <h3 class="allevents-title">
            <a href="/state-room-presents/pigeons-playing-ping-pong-3">Pigeons Playing Ping Pong</a>
        </h3>
        <div class="allevents-date">
	        Tue Mar 11
        </div>
        <div class="allevents-venue2">
            The Commonwealth Room
        </div>
        <div class="allevents-link">
            <a id="acf_url_1307_11" href="https://www.axs.com/events/751925/pigeons-playing-ping-pong-tickets?skin=stateroom" class="acf_url btn" target="_blank" rel="noopener">
                On Sale Fri 11/15
            </a>
        </div>
    </div>
</div>
'''

def parse_date_stateroom(date_str):
    date_tuple = date_str.split()
    return (month_str_to_int(date_tuple[1]), int(date_tuple[2])) 

def query_city_stateroom(venue_str):
    venue_str = venue_str.lower()
    if "presents" in venue_str:
        return "slc?"
    elif "commonwealth" in venue_str:
        return "slc"
    elif "deer" in venue_str:
        return "park city"
    elif "eccles" in venue_str:
        return "slc"
    elif "state" in venue_str:
        return "slc"
    else:
        return "slc?"

# TODO: validate all fields, have sensible fallbacks for missing fields

def process_state_room():
    print("processing state room presents ...")
    url_state_room = "https://thestateroompresents.com/state-room-presents"

    shows = []
    soup = get_html(url_state_room)
    html_events = soup.find_all("div", class_="p-3")

    if html_events:
        for html_event in html_events:
            title_h3 = html_event.find("h3", class_="allevents-title")
            if not title_h3:
                continue
            artist = title_h3.find("a").getText().strip()

            link_div = html_event.find("div", class_="allevents-link")
            if link_div:
                link_a = link_div.find("a")
                ticket_url = link_a["href"] if link_a else ""
            
            date_div = html_event.find("div", class_="allevents-date")
            if date_div:
                date = parse_date_stateroom(date_div.getText().strip())
            else:
                date = (0,0) 

            venue_div = html_event.find("div", class_="allevents-venue2")
            venue = venue_div.getText().strip()
            city = query_city_stateroom(venue)

            shows.append(
                Show(
                    artist, 
                    Show.Date(date[0], date[1]),
                    city,
                    venue,
                    ticket_url
                )
            )
            #print(f"'{artist}' '{date}' '{venue}' '{city}'")

    else:
        print(f"{url_state_room} failed") 

    print(f"\tshows found: {len(shows)}")
    return shows 


################################################################################
#
# The Complex 
#
################################################################################

'''
<div class="content">
    <a href="https://www.thecomplexslc.com/event-2562.htm" class="image-link" title="Mark Ambor - The Rockwood Tour">
        <h3>Mark Ambor - The Rockwood Tour</h3>
        <h4>Tuesday Nov 19th</h4>
        <h4>The Grand</h4>
        <p>Indie</p>
    </a>
</div>
'''

def parse_date_the_complex(date_str):
    date_re = re.compile("[a-zA-Z]+\s+([a-zA-Z]+)\s+(\d+).+")
    m = date_re.match(date_str)
    if m:
        month_str = m.groups()[0]
        day_str = m.groups()[1]
        return (month_str_to_int(month_str), int(day_str)) 
    else:
        print(f"WARN: Failed to regex match The Complex date str '{date_str}'")
        return (0,0) 


def process_the_complex():
    
    print("processing the complex ...")
    shows = []
    url_the_complex = "https://www.thecomplexslc.com/"
    
    soup = get_html(url_the_complex)
    html_events = soup.find_all("a", class_="image-link")
    if html_events:
        for html_event in html_events:
            artist = html_event.get('title')
            if not artist:
                print("\tWARN: failed to find event title")
                continue
            ticket_url = html_event.get('href') 
            date_header = html_event.find('h4')
            if date_header:
                date = parse_date_the_complex(date_header.getText().strip())
            else:
                date = (0,0) 

            venue = "the complex"
            city = "slc"

            print(f"'{artist}' '{date}' '{venue}' '{city}'")
            shows.append(
                Show(
                    artist, 
                    Show.Date(date[0], date[1]),
                    city,
                    venue,
                    ticket_url
                )
            )

    else:
        print(f"{url_the_complex} failed") 

    return shows 

################################################################################
#
# main 
#
################################################################################


shows  = process_24tix()
shows += process_state_room()
shows += process_the_complex()
generate_html(shows)

