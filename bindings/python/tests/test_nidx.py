import pytest

from nidx import NidInfo, albania

VALID_NID = "J00101999W"


class TestAlbaniaDecode:
    def test_returns_nid_info(self):
        info = albania.decode(VALID_NID)
        assert isinstance(info, NidInfo)

    def test_birthday(self):
        info = albania.decode(VALID_NID)
        assert info.birthday == "1990-01-01"

    def test_date_components(self):
        info = albania.decode(VALID_NID)
        assert info.year == 1990
        assert info.month == 1
        assert info.day == 1

    def test_sex(self):
        info = albania.decode(VALID_NID)
        assert info.sex == "M"

    def test_country(self):
        info = albania.decode(VALID_NID)
        assert info.country == "albania"

    def test_is_national(self):
        info = albania.decode(VALID_NID)
        assert info.is_national is True

    def test_female_albanian(self):
        info = albania.decode("J05115999K")
        assert info.birthday == "1990-01-15"
        assert info.sex == "F"
        assert info.is_national is True

    def test_male_foreigner(self):
        info = albania.decode("J03101999F")
        assert info.birthday == "1990-01-01"
        assert info.sex == "M"
        assert info.is_national is False

    def test_female_foreigner(self):
        info = albania.decode("J08101999P")
        assert info.birthday == "1990-01-01"
        assert info.sex == "F"
        assert info.is_national is False

    def test_case_insensitive(self):
        lower = albania.decode("j00101999w")
        upper = albania.decode("J00101999W")
        assert lower.birthday == upper.birthday
        assert lower.sex == upper.sex
        assert lower.is_national == upper.is_national

    def test_invalid_raises_value_error(self):
        with pytest.raises(ValueError):
            albania.decode("invalid")

    def test_empty_raises_value_error(self):
        with pytest.raises(ValueError):
            albania.decode("")

    def test_bad_checksum_raises_value_error(self):
        with pytest.raises(ValueError):
            albania.decode("J00101999A")

    def test_repr(self):
        info = albania.decode(VALID_NID)
        r = repr(info)
        assert "NidInfo" in r
        assert "1990-01-01" in r


class TestAlbaniaIsValid:
    def test_valid(self):
        assert albania.is_valid(VALID_NID) is True

    def test_invalid(self):
        assert albania.is_valid("invalid") is False

    def test_empty(self):
        assert albania.is_valid("") is False

    def test_bad_checksum(self):
        assert albania.is_valid("J00101999A") is False


class TestNidInfoFrozen:
    def test_attributes_are_readonly(self):
        info = albania.decode(VALID_NID)
        with pytest.raises(AttributeError):
            info.birthday = "2000-01-01"
