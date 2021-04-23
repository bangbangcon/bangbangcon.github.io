---
layout: default-2021
title: Program - !!Con 2021
---

# Conference Program

<style type="text/css">
    /* Event Types */
    .syncwatch {
        background-color: #D9DBFF;
        border-radius: 3px;
    }
    .session {
        background-color: #F5D0F2;
        border-radius: 3px;
    }
    .unconf {
        background-color: #A887A6;
        color: white;
        border-radius: 3px;
    }
    .keynote {
        background-color: #8788A8;
        color: white;
        border-radius: 3px;
    }

    /* Table */
    table {
        table-layout: fixed;
        width: 100%;
        border-collapse: collapse;

    }
    tr:nth-child(even){
        background-color: white;
    }
    td {
        padding: 0.25rem;
        border-color: white;
    }
    table, tr, td, th {
        border: 1px solid #8788A8;
    }
    th.current {
        background-color: white
    }
    .current {
        font-weight: bold;
    }

    /* Cards */
    .cards {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-gap: 1rem;
        grid-auto-rows: 400px;
    }
    .card {
        background-color: #FFFFE6;
        border-radius: 3px;
        border: 1px solid #A887A6;
        padding: 0 1rem;
        box-shadow: 2px 2px 5px 3px rgba(0,0,0,0.1);
    }
    .card.current {
        background-color: white;
    }
    .card > p {
        padding: 0.25rem 0.5rem;
    }

    @media (max-width: 900px) {
        .cards {
            grid-template-columns: 1fr 1fr;
        }
    }

    @media (max-width: 700px) {
        .cards {
            grid-template-columns: 1fr;
        }
    }

    /* Definitions */
    dfn {
        padding: 0.25rem 0.5rem;
    }

    /* Timezone */
    #timezone {
        font-family: monospace;
    }
</style>

<!-- Extra nbsp to make it look good :) -->

Timezone: &nbsp;<span id="timezone">Pacific Daylight Time (GMT -7)</span>.

All times in 24h format.

View the schedule <a href="#cards-section">Day-by-day</a> or as a <a href="#table">Table</a>

## Events

<p><dfn class="keynote">Keynote</dfn> - Our keynote speakers give a 30-40 minute talk!</p>
<p><dfn class="session">Session</dfn> - A series of 10-minute lightning talks!</p>
<p><dfn class="unconf">Unconferencing</dfn> - A make-your-own-session time!</p>
<p><dfn class="syncwatch">Syncwatch</dfn> - Rebroadcast of an earlier session!</p>

<span id="cards-section"></span>

## Schedule

<div class="cards" id="cards">

<div class="card">
<h4>Saturday, May 15</h4>
<p class="keynote"><time datetime="12:00">12:00</time> Keynote</p>
<p class="session"><time datetime="13:00">13:00</time> Session 1</p>
<p class="unconf"><time datetime="14:00">14:00</time> Unconferencing</p>
<p class="session"><time datetime="15:00">14:00</time> Session 2</p>
</div>

<div class="card">
<h4>Sunday, May 16</h4>
<p class="syncwatch"><time datetime="10:00">10:00</time> Syncwatch 1</p>
<p class="syncwatch"><time datetime="11:00">11:00</time> Syncwatch 2</p>
<hr>
<p class="session"><time datetime="17:00">17:00</time> Session 3</p>
</div>

<div class="card">
<h4>Monday, May 17</h4>
<p class="syncwatch"><time datetime="11:00">11:00</time> Syncwatch 3</p>
<hr>
<p class="session"><time datetime="17:00">17:00</time> Session 4</p>
</div>

<div class="card">
<h4>Tuesday, May 18</h4>
<p class="syncwatch"><time datetime="11:00">11:00</time> Syncwatch 4</p>
<hr>
<p class="session"><time datetime="17:00">17:00</time> Session 5</p>
</div>

<div class="card">
<h4>Wedneday, May 19</h4>
<p class="session"><time datetime="11:00">11:00</time> Session 6</p>
<hr>
<p class="syncwatch"><time datetime="17:00">17:00</time> Syncwatch 6</p>
</div>

<div class="card">
<h4>Thursday, May 20</h4>
<p class="syncwatch"><time datetime="11:00">11:00</time> Syncwatch 5</p>
<hr>
<p class="session"><time datetime="17:00">17:00</time> Session 7</p>
</div>

<div class="card">
<h4>Friday, May 21</h4>
<p class="syncwatch"><time datetime="11:00">11:00</time> Syncwatch 7</p>
<hr>
<p class="session"><time datetime="17:00">17:00</time> Session 8</p>
</div>

<div class="card">
<h4>Saturday, May 22</h4>
<p class="syncwatch"><time datetime="9:00">9:00</time> Syncwatch 8</p>
<p class="session"><time datetime="10:00">10:00</time> Session 9</p>
<p class="keynote"><time datetime="11:00">11:00</time> Keynote</p>
<hr>
<p class="syncwatch"><time datetime="17:00">17:00</time> Syncwatch 9</p>
</div>

</div>

<span id="table-section"></span>

## Table

Same info, different format :)

<table>
    <tr>
        <th></th>
        <th>Sat 15<sup>th</sup></th>
        <th>Sun 16<sup>th</sup></th>
        <th>Mon 17<sup>th</sup></th>
        <th>Tue 18<sup>th</sup></th>
        <th>Wed 19<sup>th</sup></th>
        <th>Thu 20<sup>th</sup></th>
        <th>Fri 21<sup>st</sup></th>
        <th>Sat 22<sup>nd</sup></th>
    </tr>
    <tr>
        <th><time datetime="9:00">9:00</time></th>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td class="syncwatch">Sync 8</td>
    </tr>
    <tr>
        <th><time datetime="10:00">10:00</time></th>
        <td></td>
        <td class="syncwatch">Sync 1</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td class="session">Session 9</td>
    </tr>
    <tr>
        <th><time datetime="11:00">11:00</time></th>
        <td></td>
        <td class="syncwatch">Sync 2</td>
        <td class="syncwatch">Sync 3</td>
        <td class="syncwatch">Sync 4</td>
        <td class="session">Session 6</td>
        <td class="syncwatch">Sync 5</td>
        <td class="syncwatch">Sync 7</td>
        <td class="keynote">Keynote</td>
    </tr>
    <tr>
        <th><time datetime="12:00">12:00</time></th>
        <td class="keynote">Keynote</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="13:00">13:00</time></th>
        <td class="session">Session 1</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="14:00">14:00</time></th>
        <td class="unconf">Unconf.</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="15:00">15:00</time></th>
        <td class="session">Session 2</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="16:00">16:00</time></th>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="17:00">17:00</time></th>
        <td></td>
        <td class="session">Session 3</td>
        <td class="session">Session 4</td>
        <td class="session">Session 5</td>
        <td class="syncwatch">Sync 6</td>
        <td class="session">Session 7</td>
        <td class="session">Session 8</td>
        <td class="syncwatch">Sync 9</td>
    </tr>
</table>

<script>
    // Get tz as a string, e.g. "Eastern Daylight Time".
    const dateStr = new Date().toString();
    const timezone = dateStr.substring(dateStr.indexOf('(') + 1, dateStr.length - 1);
    document.getElementById('timezone').innerHTML = timezone + ' (detected)';

    // Replace time elements with tz-adjusted hours.
    Array.from(document.getElementsByTagName('time')).forEach(timeTag => {
        const d = new Date(`May 1, 2020 ${timeTag.getAttribute('datetime')}:00 GMT-0700`);
        timeTag.innerHTML = `${d.getHours()}:00`;
    });

    // Highlight current day (getDate gets day of month!)
    const currentDay = new Date().getDate();
    // getMonth is 0-based! May == 4.
    if (new Date().getMonth() == 4) {
        if (currentDay >= 15 && currentDay <= 22) {
            const dayIndex = (currentDay - 15);
            // Highlight table columns.
            Array.from(document.getElementsByTagName('tr')).forEach(row => {
                row.children.item(dayIndex + 1).classList.add('current');
            })
            // Highlight card.
            document.getElementById('cards').children.item(dayIndex).classList.add('current');
        }
    }
</script>
