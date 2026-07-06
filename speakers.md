---
layout: default-end
title: Speaker Archive - !!Con
---

<p style="text-align: center;">
  **Quick Links:**
  [Speakers](speakers.html)
  &middot;
  [Talks*](talks.html)
  &middot;
  [Past Sponsorships](sponsors.html)
  &middot;
  [Conduct](conduct.html)
</p>


<style>
  #speaker-count {
    font-size: 1.2em;
    font-weight: bold;
    color: #301934;
    margin-bottom: 15px;
    text-align: center;
  }
  .filter-controls {
    margin-bottom: 30px;
    display: flex;
    flex-wrap: wrap;
    gap: 15px;
    background: #fdfdfd;
    padding: 20px;
    border-radius: 15px;
    border: 1px solid #ddd;
    box-shadow: 0 4px 6px rgba(0,0,0,0.05);
  }
  .filter-controls input[type="text"], .filter-controls select {
    padding: 10px;
    border-radius: 8px;
    border: 1px solid #ccc;
    flex-grow: 1;
    min-width: 200px;
    font-size: 16px;
  }
  .speaker-card {
    margin-bottom: 40px;
    padding-bottom: 20px;
    border-bottom: 1px solid #eee;
    overflow: hidden; /* Clearfix for floating image */
  }
  .year-labels {
    margin-bottom: 10px;
    display: flex;
    gap: 5px;
    flex-wrap: wrap;
  }
  .speaker-card .year-label {
    display: inline-block;
    background: #301934;
    color: white;
    padding: 2px 10px;
    border-radius: 4px;
    font-size: 0.9em;
  }
  .speaker-card h3 {
    margin-top: 0;
    color: #301934;
    font-size: 1.5em;
  }
  .speaker-card .talk-title {
    font-weight: bold;
    font-size: 1.2em;
    margin-bottom: 10px;
  }
  .speaker-card .speaker-bio {
    margin-top: 15px;
    color: #333;
    font-size: 1.1em;
  }
  .speaker-card .talk-description {
    font-size: 1.1em;
    line-height: 1.5;
  }
  .hidden {
    display: none;
  }
</style>

# Speakers

<p id="speaker-count">Total entries: {{ site.data.all_talks | size }}</p>

<div class="filter-controls" markdown="0">
  <input type="text" id="search-input" placeholder="Search speaker, talk title, or bio...">
  <select id="year-filter">
    <option value="">All Years</option>
    {% assign years = "2024,2022,2021,2020,2019,2018,2017,2016,2015,2014" | split: "," %}
    {% for year in years %}<option value="{{ year }}">{{ year }}</option>{% endfor %}
  </select>
  <select id="sort-order">
    <option value="year-asc">Year (Oldest first)</option>
    <option value="year-desc">Year (Newest first)</option>
    <option value="author">Speaker Name</option>
  </select>
</div>

<div id="speaker-grid" markdown="0">
{% assign all_talks = site.data.all_talks %}
{% for talk in all_talks %}
  <div class="speaker-card" 
       data-year="{{ talk.year }}" 
       data-author="{{ talk.author | downcase | escape }}" 
       data-title="{{ talk.title | downcase | escape }}" 
       data-bio="{{ talk.speaker_bio | downcase | escape }}"
       data-order="{{ talk.order }}">
    
    <div class="year-labels">
      {% for y in talk.all_years %}
        <span class="year-label">{{ y }}</span>
      {% endfor %}
    </div>
    
    {% if talk.speaker_photo %}
      <img src="images/speakers/{{ talk.speaker_photo }}" alt="{{ talk.author | escape }}" class="speaker-img">
    {% else %}
      <img src="images/logo.png" alt="{{ talk.author | escape }}" class="speaker-img">
    {% endif %}

    <h3>{{ talk.author }}</h3>
  
<!--   
    {% if talk.title != "" %}
    <p class="talk-title"><strong>{{ talk.title }}</strong></p>
    {% endif %}
    
    {% if talk.description %}
    <div class="talk-description">
      {{ talk.description | markdownify }}
    </div>
    {% endif %}
--> 
    {% if talk.speaker_bio %}
    <div class="speaker-bio">
      {{ talk.speaker_bio | markdownify }}
    </div>
    {% endif %}
  </div>
{% endfor %}
</div>

<script>
  const searchInput = document.getElementById('search-input');
  const yearFilter = document.getElementById('year-filter');
  const sortOrder = document.getElementById('sort-order');
  const speakerGrid = document.getElementById('speaker-grid');
  const speakerCards = Array.from(document.querySelectorAll('.speaker-card'));
  const speakerCountDisplay = document.getElementById('speaker-count');
  const totalEntries = speakerCards.length;

  function sortSpeakers() {
    const order = sortOrder.value;
    const sortedCards = [...speakerCards].sort((a, b) => {
      if (order === 'year-desc' || order === 'year-asc') {
        const yearA = parseInt(a.getAttribute('data-year'));
        const yearB = parseInt(b.getAttribute('data-year'));
        if (yearA !== yearB) {
          return order === 'year-desc' ? yearB - yearA : yearA - yearB;
        }
        const orderA = parseInt(a.getAttribute('data-order') || 0);
        const orderB = parseInt(b.getAttribute('data-order') || 0);
        return orderA - orderB;
      } else if (order === 'author') {
        return a.getAttribute('data-author').localeCompare(b.getAttribute('data-author'));
      }
      return 0;
    });

    sortedCards.forEach(card => speakerGrid.appendChild(card));
  }

  function filterSpeakers() {
    const searchTerm = searchInput.value.toLowerCase();
    const selectedYear = yearFilter.value;

    let visibleCount = 0;
    speakerCards.forEach(card => {
      const author = card.getAttribute('data-author');
      const title = card.getAttribute('data-title');
      const bio = card.getAttribute('data-bio');
      const year = card.getAttribute('data-year');

      const matchesSearch = author.includes(searchTerm) || title.includes(searchTerm) || bio.includes(searchTerm);
      const matchesYear = selectedYear === "" || year === selectedYear;

      if (matchesSearch && matchesYear) {
        card.classList.remove('hidden');
        visibleCount++;
      } else {
        card.classList.add('hidden');
      }
    });

    if (visibleCount === totalEntries) {
      speakerCountDisplay.textContent = `Total entries: ${totalEntries}`;
    } else {
      speakerCountDisplay.textContent = `Showing ${visibleCount} of ${totalEntries} entries`;
    }
  }

  searchInput.addEventListener('input', filterSpeakers);
  yearFilter.addEventListener('change', filterSpeakers);
  sortOrder.addEventListener('change', sortSpeakers);
</script>
