# TODO
Import-Module "./images.psm1"

$container = Get-Container

$baseName = Get-ImageBaseName

$text = docker images | Select-String $baseName | Out-String

$images = $text.Trim().Split([Environment]::NewLine)

foreach ($image in $images) {
    $parts = [regex]::split($image, "\s+")
    $name = $parts[0]

    if ($name -ne $baseName) {
        continue
    }

    $tag = $parts[1]
    $sha = $parts[2]

    Write-Host "Deleting image ${name}:${tag} with sha $sha..."

    docker rmi $sha
}

Write-Host "Done..."