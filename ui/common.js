function showMenu()
{
    let mainMenu = document.getElementById('main_menu')
    if (mainMenu.style.visibility != "visible")
    {
        mainMenu.style.visibility="visible";
    }
    else
    {
        mainMenu.style.visibility="hidden";
    }
}
