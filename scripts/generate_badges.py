#!/usr/bin/env python3
"""
Custom badge generator for Pan-African FX Grid.
Colors inspired by pan-African palette: gold, green, red, black.
"""
import os

BADGES_DIR = "badges"

# African-inspired color palette
COLORS = {
    "gold":     "#D4A017",
    "green":    "#006B3F",
    "red":      "#C8102E",
    "black":    "#1A1A1A",
    "white":    "#F5F5F0",
    "kente_gold": "#E8B80D",
    "kente_blue": "#1E3A5F",
}

BADGE_TEMPLATES = [
    {
        "label": "built by",
        "message": "Charles Mfouapon",
        "color": "black",
        "labelColor": "gold",
        "style": "for-the-badge",
    },
    {
        "label": "region",
        "message": "Pan-African",
        "color": "green",
        "labelColor": "kente_gold",
        "style": "for-the-badge",
    },
    {
        "label": "settlement",
        "message": "Mobile Money Rails",
        "color": "red",
        "labelColor": "white",
        "style": "for-the-badge",
    },
    {
        "label": "consensus",
        "message": "Byzantine Fault Tolerant",
        "color": "black",
        "labelColor": "gold",
        "style": "for-the-badge",
    },
    {
        "label": "arbitrage",
        "message": "Real-time Detection",
        "color": "green",
        "labelColor": "white",
        "style": "for-the-badge",
    },
    {
        "label": "currencies",
        "message": "XOF XAF NGN KES GHS ZAR",
        "color": "kente_blue",
        "labelColor": "kente_gold",
        "style": "for-the-badge",
    },
    {
        "label": "rust",
        "message": "Core Engine",
        "color": "black",
        "labelColor": "white",
        "logo": "rust",
        "style": "for-the-badge",
    },
    {
        "label": "python",
        "message": "Quant Models",
        "color": "kente_blue",
        "labelColor": "kente_gold",
        "logo": "python",
        "style": "for-the-badge",
    },
    {
        "label": "typescript",
        "message": "Dashboard",
        "color": "black",
        "labelColor": "gold",
        "logo": "typescript",
        "style": "for-the-badge",
    },
]

def generate_shields_url(label, message, color, label_color=None, style=None, logo=None):
    """Generate a shields.io URL for the badge."""
    base = "https://img.shields.io/badge/"
    label_part = label.replace(" ", "%20")
    message_part = message.replace(" ", "%20")
    url = f"{base}{label_part}-{message_part}-{color}"
    
    params = []
    if label_color:
        params.append(f"labelColor={label_color}")
    if style:
        params.append(f"style={style}")
    if logo:
        params.append(f"logo={logo}")
    
    if params:
        url += "?" + "&".join(params)
    
    return url

def generate_markdown():
    """Generate markdown badge tags for README."""
    lines = ["<!-- Custom badges for Pan-African FX Grid -->", ""]
    
    for badge in BADGE_TEMPLATES:
        url = generate_shields_url(
            label=badge["label"],
            message=badge["message"],
            color=COLORS[badge["color"]],
            label_color=COLORS.get(badge.get("labelColor", "")),
            style=badge.get("style", ""),
            logo=badge.get("logo", ""),
        )
        alt = f"{badge['label']}: {badge['message']}"
        lines.append(f"![{alt}]({url})")
    
    return "\n".join(lines)

def generate_html():
    """Generate HTML badge tags."""
    lines = ['<div align="center">', ""]
    
    for badge in BADGE_TEMPLATES:
        url = generate_shields_url(
            label=badge["label"],
            message=badge["message"],
            color=COLORS[badge["color"]],
            label_color=COLORS.get(badge.get("labelColor", "")),
            style=badge.get("style", ""),
            logo=badge.get("logo", ""),
        )
        alt = f"{badge['label']}: {badge['message']}"
        lines.append(f'  <img src="{url}" alt="{alt}" />')
    
    lines.append("")
    lines.append("</div>")
    return "\n".join(lines)

if __name__ == "__main__":
    os.makedirs(BADGES_DIR, exist_ok=True)
    
    # Generate markdown badges
    md = generate_markdown()
    with open(f"{BADGES_DIR}/badges.md", "w") as f:
        f.write(md)
    
    # Generate HTML badges
    html = generate_html()
    with open(f"{BADGES_DIR}/badges.html", "w") as f:
        f.write(html)
    
    # Print for README
    print("Copy these badges to your README:")
    print()
    print(md)
    
    print()
    print("Badges generated in badges/ directory.")
