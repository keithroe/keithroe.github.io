#!/usr/bin/env python3
import re 
import itertools
import requests
from bs4 import BeautifulSoup
import functools
from datetime import date
import datetime

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
        <p>
        <a href="#about">about this page</a>
        </p>
        <table id=events_table></table>
        <div id="about"></div>
        <p id="about">
            <pre id="about"> </pre>
        </p>
    </main>
  </body>
</html>
'''




################################################################################
#
# Show data
#
################################################################################


class Show:

    def __init__(self, artist, date, city, venue, url):
        self.artist = artist
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
        
    def in_past(self):
        return self.date < date.today()

    def __eq__(self, other):
        return self.date == other.date and self.artist == other.artist

    def __lt__(self, other):
        return self.date < other.date or self.date == other.date and self.artist < other.artist
    
    def __hash__(self):
        return hash((self.artist, self.date))


################################################################################
#
# util 
#
################################################################################

def get_html(url):
    try: 
        headers = {
            'authority': 'www.google.com',
            'accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7',
            'accept-language': 'en-US,en;q=0.9',
            'cache-control': 'max-age=0',
            #'cookie': 'SID=ZAjX93QUU1NMI2Ztt_dmL9YRSRW84IvHQwRrSe1lYhIZncwY4QYs0J60X1WvNumDBjmqCA.; __Secure- #..,
            'sec-ch-ua': '"Not/A)Brand";v="99", "Google Chrome";v="115", "Chromium";v="115"',
            'sec-ch-ua-arch': '"x86"',
            'sec-ch-ua-bitness': '"64"',
            'sec-ch-ua-full-version': '"115.0.5790.110"',
            'sec-ch-ua-full-version-list': '"Not/A)Brand";v="99.0.0.0", "Google Chrome";v="115.0.5790.110", "Chromium";v="115.0.5790.110"',
            'sec-ch-ua-mobile': '?0',
            'sec-ch-ua-model': '""',
            'sec-ch-ua-platform': 'Windows',
            'sec-ch-ua-platform-version': '15.0.0',
            'sec-ch-ua-wow64': '?0',
            'sec-fetch-dest': 'document',
            'sec-fetch-mode': 'navigate',
            'sec-fetch-site': 'same-origin',
            'sec-fetch-user': '?1',
            'upgrade-insecure-requests': '1',
            'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36',
            'x-client-data': '#..',
        }

        response = requests.get(url, headers=headers)
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


def make_date(month, day):
    today = date.today()
    if month < today.month or month == today.month and day < today.day:
        year = today.year + 1
    else:
        year = today.year
    return date(year, month, day)


def generate_html(shows):
    shows = sorted(list(set(shows)))
    soup = BeautifulSoup(html_template, "html.parser")
    table = soup.find("table")
    with open("index.html", "w", encoding='utf-8') as file:
        last_date = None
        for show in shows:
            if show.in_past():
                continue
            row = soup.new_tag("tr")
            # date
            if last_date == None or last_date != show.date:
                last_date = show.date
                month_str = month_int_to_str(show.date.month)
                row.append(BeautifulSoup(f"<pre><td>{month_str} {show.date.day:02d}&nbsp;&nbsp;</td></pre>", "html.parser"))
                #row.append(BeautifulSoup(f"<pre><td>{show.date}&nbsp;&nbsp;</td></pre>", "html.parser"))
            else:
                row.append(BeautifulSoup(f"<td></td>", "html.parser"))

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
        
        about_pre = soup.find("pre", id="about")
        about_pre.append(f"\nabout this page:\n")
        about_pre.append(f"\tmissing venues or feedback: slcshowsnet AT gmail DOT com\n")
        about_pre.append(f"\tgenerated on: {date.today()}\n---\n\n")
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
        return make_date(month_str_to_int(month_str), int(day_str)) 
    else:
        print(f"WARN: Failed to regex match 24tix date str '{date_str}'")
        return None 


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
                    date = None 
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
                        date,
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
    return make_date(month_str_to_int(date_tuple[1]), int(date_tuple[2])) 


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
                date = None 

            venue_div = html_event.find("div", class_="allevents-venue2")
            venue = venue_div.getText().strip()
            city = query_city_stateroom(venue)

            shows.append(
                Show(
                    artist, 
                    date,
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
        return make_date(month_str_to_int(month_str), int(day_str)) 
    else:
        print(f"WARN: Failed to regex match The Complex date str '{date_str}'")
        return None 


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
                date = None 

            venue = "the complex"
            city = "slc"

            #print(f"'{artist}' '{date}' '{venue}' '{city}'")
            shows.append(
                Show(
                    artist, 
                    date,
                    city,
                    venue,
                    ticket_url
                )
            )

    else:
        print(f"{url_the_complex} failed") 

    print(f"\tshows found: {len(shows)}")
    return shows 

################################################################################
#
# the depot 
#
################################################################################

'''
<div class="chakra-card__footer css-6nvkkc">
 <div class="css-1cbuemh">
  <div class="css-1u2896s">
   <p class="chakra-text css-zvlevn">
    Nero
   </p>
   <p class="chakra-text css-1g5zdf0">
    Sat Nov 16, 2024
   </p>
   <div class="css-7timbt">
    <div class="css-0">
     <a class="chakra-button css-1asqcxu" href="https://concerts.livenation.com/nero-salt-lake-city-utah-11-16-2024/event/1E0060BFC2A05247" target="_blank">
      Buy Tickets
     </a>
    </div>
    <div class="css-14dycje">
     <button class="chakra-button css-16uafp" type="button">
      More Info
     </button>
    </div>
   </div>
  </div>
 </div>
 <div class="css-27cwld">
  <button aria-label="More Info" class="css-1v6uy3d">
   <i aria-hidden="true" class="icn icn-ellipses __className_f3e3e6 css-1eqjgjs" focusable="false">
   </i>
  </button>
 </div>
 <time class="css-kfisjo">
  <p class="chakra-text css-1yp7tc1">
   Sat
  </p>
  <p class="chakra-text date-box-date css-go0khb">
   16
  </p>
  <p class="chakra-text css-1yp7tc1">
   Nov
  </p>
 </time>
 <div class="css-l1pvlg">
  <a class="chakra-linkbox__overlay css-a0kmza" href="https://concerts.livenation.com/nero-salt-lake-city-utah-11-16-2024/event/1E0060BFC2A05247" target="_blank">
   Nero
  </a>
 </div>
</div>
'''

def parse_date_the_depot(month_str, day_str):
    return make_date(month_str_to_int(month_str), int(day_str)) 


def process_the_depot():
    from datetime import datetime
    from datetime import date
    from dateutil.relativedelta import relativedelta
    
    print("processing the depot ...")
    shows = []
    url_template = "https://www.depotslc.com/shows?start={}"
    
    latest_date = date.today()

    while True: 
        url_the_depot = url_template.format(str(latest_date))
        print("\tsearching: {}".format(url_the_depot))

        num_events = 0
        soup = get_html(url_the_depot)
        html_events = soup.find_all("div", class_="chakra-card__footer")
        if html_events:
            print(f"\tfound {len(html_events)} events")
            for html_event in html_events:
                ps = html_event.find_all('p')
                date_text = ps[1].getText()
                date_text = " ".join(date_text.split()[1:])
                latest_date = datetime.strptime(date_text, "%b %d, %Y").date()
                latest_date = latest_date + relativedelta(days=1)
                link = html_event.find('a', class_='chakra-linkbox__overlay')
                if not link:
                    print("\tWARN: failed to find linkbox overlay")
                    continue
                artist = link.getText().strip()
                ticket_url = link.get('href') 

                date_time = html_event.find('time')
                if date_time:
                    ps = date_time.find_all('p')
                    day = ps[1].getText().split("-")[0]
                    month = ps[2].getText()
                    date = parse_date_the_depot(month, day)
                else:
                    date = None 

                venue = "the depot"
                city = "slc"

                #print(f"'{artist}' '{date}' '{venue}' '{city}'")
                shows.append(
                    Show(
                        artist, 
                        date,
                        city,
                        venue,
                        ticket_url
                    )
                )

        else:
            print(f"{url_the_depot} failed") 
            break

    print(f"\tshows found: {len(shows)}")
    return shows 

################################################################################
#
# aces high 
#
################################################################################

'''
<div  class="tribe-common-g-row tribe-events-calendar-list__event-row" >
    <div class="tribe-events-calendar-list__event-date-tag tribe-common-g-col">
        <time class="tribe-events-calendar-list__event-date-tag-datetime" datetime="2024-12-05" aria-hidden="true">
            <span class="tribe-events-calendar-list__event-date-tag-weekday">
                Thu        
            </span>
            <span class="tribe-events-calendar-list__event-date-tag-daynum tribe-common-h5 tribe-common-h4--min-medium">
                5
            </span>
        </time>
    </div>

    <div class="tribe-events-calendar-list__event-wrapper tribe-common-g-col">
        <article  class="tribe-events-calendar-list__event tribe-common-g-row tribe-common-g-row--gutters post-23925 tribe_events type-tribe_events status-publish hentry" >
            <div class="tribe-events-calendar-list__event-details tribe-common-g-col">
                <header class="tribe-events-calendar-list__event-header">
                    <div class="tribe-events-calendar-list__event-datetime-wrapper tribe-common-b2">
                        <time class="tribe-events-calendar-list__event-datetime" datetime="2024-12-05">
                            <span class="tribe-event-date-start">December 5 @ 8:00 pm</span> 
                            - 
                            <span class="tribe-event-time">11:59 pm</span>
                            <span class='timezone'> MST </span>
                        </time>
                    </div>
                    <h3 class="tribe-events-calendar-list__event-title tribe-common-h6 tribe-common-h4--min-medium">
                        <a
                            href="https://aceshighsaloon.com/event/public-serpent/"
                            title="Public Serpent"
                            rel="bookmark"
                            class="tribe-events-calendar-list__event-title-link tribe-common-anchor-thin"
                        >
                             Public Serpent    
                        </a>
                    </h3>
                </header>
            </div>
        </article>
    </div>
</div>
'''


def parse_date_aces_high(date_str):
    return date.fromisoformat(date_str)


def process_aces_high():
    
    print("processing aces_high ...")
    shows = []
    url_aces_high = "https://aceshighsaloon.com/events/list/"
    
    soup = get_html(url_aces_high)
    html_events = soup.find_all("div", class_="tribe-events-calendar-list__event-row")
    if html_events:
        for html_event in html_events:

            link = html_event.find('a')
            if not link:
                print("\tWARN: failed to find event link")
                continue

            ticket_url = link.get('href') 
            artist = link.getText().strip()

            time_tag = html_event.find('time')
            if time_tag:
                date = parse_date_aces_high(time_tag.get('datetime'))
            else:
                print("\tWARN: failed to find time tag")
                date = None 

            venue = "aces high"
            city = "slc"

            #print(f"'{artist}' '{date}' '{venue}' '{city}'")
            shows.append(
                Show(
                    artist, 
                    date,
                    city,
                    venue,
                    ticket_url
                )
            )

    else:
        print(f"{url_aces_high} failed") 

    print(f"\tshows found: {len(shows)}")
    return shows 

################################################################################
#
# the union 
#
################################################################################
import urllib.parse

'''
<div class="eventlist-column-info">
  <h1 class="eventlist-title">
    <a href="/upcomingevents/finneas" class="eventlist-title-link">
      FINNEAS - For Cryin' Out Loud!: The Tour
    </a>
  </h1>
  <ul class="eventlist-meta event-meta">
    <li class="eventlist-meta-item eventlist-meta-date event-meta-item">
      <time class="event-date" datetime="2025-03-02">Sunday, March 2, 2025</time>
    </li>
    <li class="eventlist-meta-item eventlist-meta-time event-meta-item">
      <span class="event-time-12hr">
        <time class="event-time-12hr-start" datetime="2025-03-02">6:30 PM</time>
        <span class="event-datetime-divider"></span>
        <time class="event-time-12hr-end" datetime="2025-03-02">10:30 PM</time>
      </span>
      <span class="event-time-24hr">
        <time class="event-time-24hr-start" datetime="2025-03-02">18:30</time>
        <span class="event-datetime-divider"></span>
        <time class="event-time-12hr-end" datetime="2025-03-02">22:30</time>
      </span>
    </li>
      <li class="eventlist-meta-item eventlist-meta-address event-meta-item">
          The Union Event Center
        <a href="http://maps.google.com?q=235 North 500 West Salt Lake City, Utah, 84116 United States" class="eventlist-meta-address-maplink" target="_blank">(map)</a>
      </li>
    <li class="eventlist-meta-item eventlist-meta-export event-meta-item">
      <a href="http://www.google.com/calendar/event?action=TEMPLATE&text=FINNEAS%20-%20For%20Cryin%27%20Out%20Loud%21%3A%20The%20Tour&dates=20250303T013000Z/20250303T053000Z&location=235%20North%20500%20West%2C%20Salt%20Lake%20City%2C%20Utah%2C%2084116%2C%20United%20States" class="eventlist-meta-export-google">Google Calendar</a>
      <span class="eventlist-meta-export-divider"></span>
      <a href="/upcomingevents/finneas?format=ical" class="eventlist-meta-export-ical">ICS</a>
    </li>
  </ul>
  <div class="eventlist-excerpt"><p class=""><strong>ALL AGES</strong></p></div>
  <a href="/upcomingevents/finneas" class="eventlist-button sqs-button-element--primary">
    View Event &#8594;
  </a>
  <div class="eventlist-actions">
    <span class="sqs-simple-like" data-item-id="66fec350d7a785280bdefe8e" data-like-count="0">
      <span class="like-icon"></span>
      <span class="like-count"></span>
    </span>
    <span class="squarespace-social-buttons inline-style" data-system-data-id="1728408791646-BGQO7U76G82Q40JHBM35" data-asset-url="https://images.squarespace-cdn.com/content/v1/5a48752132601ea2c0890ed8/1728408791646-BGQO7U76G82Q40JHBM35/Static_Social-Instagram_1080x1080_Finneas_2025_Regional_TheUnionEventCenter_0302.jpg" data-record-type="12" data-full-url="/upcomingevents/finneas" data-title="FINNEAS - For Cryin' Out Loud!: The Tour">
    </span>
  </div>
</div>
'''


def parse_date_the_union(date_str):
    return date.fromisoformat(date_str)


def process_the_union():
    
    print("processing the union ...")
    shows = []
    url_the_union = "https://theunioneventcenter.com/"
    
    soup = get_html(url_the_union)
    html_events = soup.find_all("div", class_="eventlist-column-info")
    if html_events:
        for html_event in html_events:
            link = html_event.find('a')
            if not link:
                print("\tWARN: failed to find event link")
                continue
            artist = link.getText().strip()
            relative_url = link.get('href')
            ticket_url = urllib.parse.urljoin(url_the_union, relative_url)  

            time = html_event.find('time')
            if time:
                date_str = time.get('datetime')
                date = parse_date_the_union(date_str)
            else:
                date = None

            venue = "the union"
            city = "slc"

            #print(f"'{artist}' '{date}' '{venue}' '{city}'")
            shows.append(
                Show(
                    artist, 
                    date,
                    city,
                    venue,
                    ticket_url
                )
            )
    else:
        print(f"{url_the_union} failed") 

    print(f"\tshows found: {len(shows)}")
    return shows 

################################################################################
#
# soundwell 
#
################################################################################

'''
<div class="event-wrapper" >
    <div class="event event-start-date">
        <h4 class="date month">Nov</h4>
        <h3 class="date number">22</h3>
    </div>
    <div class="event-image-container">
        <a href="https://tixr.com/e/96551" class="event event-image" target="_blank">
            <img src="https://soundwellslc.com/wp-content/uploads/2024/02/005_Matthew_Yoscary23-copy_600x600_acf_cropped-1.jpg" loading="lazy"/>
        </a>
    </div>
    <div class="info">
        <div class="event event-info">
            <h3 class="event-name">HOMESHAKE</h3>
            <h4 class="event-feature">GREEN-HOUSE</h4>
            <p>
                <br/>
                <h4>
                    <span class="event-feature event-age">AA</span>
                    <span class="event-feature"> | Doors At </span>
                    <span class="event-feature event-time">7:00 pm</span>
                    <span class="event-feature event-price"> | $22+</span>
                </h4>
                <br/>
            </p>
            <div class="description">
            </div>
        </div>
        <br/>
        <div class="event event-tickets">
            <a href="https://tixr.com/e/96551" class="button event-link on-sale" target="_blank">
                GET TICKETS!                                                                                                                                                                                                                                <span></span>
            </a>
        </div>
    </div>
</div>
'''

def parse_date_soundwell(month_str, day_str):
    return make_date(month_str_to_int(month_str), int(day_str)) 


def process_soundwell():
    
    print("processing soundwell ...")
    shows = []
    url_soundwell = "https://soundwellslc.com/events/"
    
    soup = get_html(url_soundwell)
    html_events = soup.find_all("div", class_="event-wrapper")
    if html_events:
        for html_event in html_events:

            name_header = html_event.find("h3", class_="event-name")
            if not name_header:
                print("\tWARN: failed to find event name")
                continue
            artist = name_header.getText().strip()

            ticket_div= html_event.find("div", class_="event-tickets")
            if ticket_div:
                link = ticket_div.find('a')
                if link:
                    ticket_url = link.get('href')
                else:
                    ticket_url = None
            else:
                ticket_url = None 

            date_div = html_event.find("div", class_="event-start-date")
            if date_div:
                month_str = date_div.find("h4").getText().strip()
                day_str = date_div.find("h3").getText().strip()
                date = parse_date_soundwell(month_str, day_str)
            else:
                date = None 

            venue = "soundwell"
            city = "slc"

            #print(f"'{artist}' '{date}' '{venue}' '{city}'")
            shows.append(
                Show(
                    artist, 
                    date,
                    city,
                    venue,
                    ticket_url
                )
            )

    else:
        print(f"{url_soundwell} failed") 

    print(f"\tshows found: {len(shows)}")
    return shows 


################################################################################
#
# main 
#
################################################################################


shows = []
shows += process_24tix()
shows += process_state_room()
shows += process_the_complex()
shows += process_the_depot()
shows += process_aces_high()
shows += process_the_union()
shows += process_soundwell()
generate_html(shows)

